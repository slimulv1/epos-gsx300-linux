#!/usr/bin/env python3
# DEPRECATED 2026-09-12: superseded by recdis.py (same folder), which adds the
# missing RMB/SMB/BBR/BBS bit-op table (0x07-0xF7) per the official WDC
# W65C02S datasheet. Kept for reference only — use recdis.py for new work.
"""W65C02S full disassembler (CMOS/WDC extended: undefined=C0-CF->NOP, included STZ/TSB/TRB/BRA/INA/DEA/PHX/PHY/SBCdz etc.)
Linear disassembly from given offset, marking code vs data by known patterns."""
import sys

# W65C02S opcode map: (mode, mnemonic)
# modes: imp acc imm zp zpx zpy rel abs absx absy ind indx indy izp absind
M = {}
def S(op, mode, mn): M[op] = (mode, mn)
# --- implied ---
for op, mn in {0x00:'BRK',0x08:'PHP',0x18:'CLC',0x28:'PLP',0x38:'SEC',0x40:'RTI',0x48:'PHA',
              0x58:'CLI',0x60:'RTS',0x68:'PLA',0x88:'DEY',0x98:'TYA',0xa8:'TAY',0xb8:'CLV',
              0xc8:'INY',0xd8:'CLD',0xe8:'INX',0xf8:'SED',0xaa:'TAX',0x8a:'TXA',0x9a:'TXS',
              0xba:'TSX',0xca:'DEX',0xea:'NOP',0x1a:'INA',0x3a:'DEA',0x5a:'PHY',0x7a:'PLY',
              0xda:'PHX',0xfa:'PLX',0xeb:'SBC'}.items(): S(op,'imp',mn)
# --- acc ---
for op, mn in {0x0a:'ASL',0x2a:'ROL',0x4a:'LSR',0x6a:'ROR'}.items(): S(op,'acc',mn)
# --- branch rel ---
for op, mn in {0x10:'BPL',0x30:'BMI',0x50:'BVC',0x70:'BVS',0x90:'BCC',0xb0:'BCS',0xd0:'BNE',0xf0:'BEQ',0x80:'BRA'}.items(): S(op,'rel',mn)
# --- immediate ---
for op, mn in {0x09:'ORA',0x29:'AND',0x49:'EOR',0x69:'ADC',0x89:'BIT',0xa9:'LDA',0xc9:'CMP',0xe9:'SBC',0xa0:'LDY',0xe0:'CPX',0xc0:'CPY',0xa2:'LDX'}.items(): S(op,'imm',mn)
# --- zp ---
for op, mn in {0x04:'TSB',0x05:'ORA',0x06:'ASL',0x14:'TRB',0x15:'ORA',0x16:'ASL',0x24:'BIT',0x25:'AND',0x26:'ROL',
               0x34:'BIT',0x35:'AND',0x36:'ROL',0x44:'TSB',0x45:'EOR',0x46:'LSR',0x54:'TRB',0x55:'EOR',0x56:'LSR',
               0x64:'STZ',0x65:'ADC',0x66:'ROR',0x74:'STZ',0x75:'ADC',0x76:'ROR',0x84:'STY',0x85:'STA',0x86:'STX',
               0x94:'STY',0x95:'STA',0x96:'STX',0xa4:'LDY',0xa5:'LDA',0xa6:'LDX',0xb4:'LDY',0xb5:'LDA',0xb6:'LDX',
               0xc4:'CPY',0xc5:'CMP',0xc6:'DEC',0xd4:'CPY',0xd5:'CMP',0xd6:'DEC',0xe4:'CPX',0xe5:'SBC',0xe6:'INC',
               0xf4:'CPX',0xf5:'SBC',0xf6:'INC',0x12:'ORA',0x32:'AND',0x52:'EOR',0x72:'ADC',0x92:'STA',0xb2:'LDA',
               0xd2:'CMP',0xf2:'SBC'}.items():
    mode = 'zpx' if op % 0x10 in (6,0x14) else ('zpy' if op in (0x96,0xb6) else ('izp' if op % 0x10 == 0x02 else ('zpy' if op%0x10 in (0x10,) else 'zp')))
    # refine: STY zpx = 0x94, STX zpy = 0x96, LDY zpx 0xb4, LDX zpy 0xb6
    if op in (0x94,0xb4): mode = 'zpx'
    if op in (0x96,0xb6): mode = 'zpy'
    if op in (0x12,0x32,0x52,0x72,0x92,0xb2,0xd2,0xf2): mode = 'izp'
    S(op, mode, mn)
# --- abs ---
for op, mn in {0x0c:'TSB',0x0d:'ORA',0x0e:'ASL',0x1c:'TRB',0x1d:'ORA',0x1e:'ASL',0x2c:'BIT',0x2d:'AND',0x2e:'ROL',
               0x3c:'BIT',0x3d:'AND',0x3e:'ROL',0x4c:'JMP',0x4d:'EOR',0x4e:'LSR',0x5c:'JMP',0x5d:'EOR',0x5e:'LSR',
               0x6c:'JMP',0x6d:'ADC',0x6e:'ROR',0x7c:'JMP',0x7d:'ADC',0x7e:'ROR',0x8c:'STY',0x8d:'STA',0x8e:'STX',
               0x9c:'STZ',0x9d:'STA',0x9e:'STZ',0xac:'LDY',0xad:'LDA',0xae:'LDX',0xbc:'LDY',0xbd:'LDA',0xbe:'LDX',
               0xcc:'CPY',0xcd:'CMP',0xce:'DEC',0xdc:'CPY',0xdd:'CMP',0xde:'DEC',0xec:'CPX',0xed:'SBC',0xee:'INC',
               0xfc:'CPX',0xfd:'SBC',0xfe:'INC',0x20:'JSR'}.items():
    mode = 'absx' if op % 0x10 == 0x0d else ('absind' if op % 0x10 == 0x0c else 'abs')
    if op in (0x1d,0x3d,0x5d,0x7d,0x9d,0xbd,0xdd,0xfd): mode='absx'
    if op in (0x9e,): mode='absx'
    if op in (0x7c,0x5c): mode='abxind'  # JMP (abs,X) on W65C02
    if op in (0x6c,): mode='absind'
    S(op, mode, mn)
# --- idx indirect / indirect idx ---
for op, mn in {0x21:'AND',0x41:'EOR',0x61:'ADC',0x81:'STA',0xa1:'LDA',0xc1:'CMP',0xe1:'SBC',0x01:'ORA'}.items(): S(op,'indx',mn)
for op, mn in {0x11:'ORA',0x31:'AND',0x51:'EOR',0x71:'ADC',0x91:'STA',0xb1:'LDA',0xd1:'CMP',0xf1:'SBC'}.items(): S(op,'indy',mn)

LEN = {'imp':1,'acc':1,'imm':2,'zp':2,'zpx':2,'zpy':2,'rel':2,'abs':3,'absx':3,'absy':3,'ind':1,'indx':2,'indy':2,'izp':2,'absind':3,'abxind':3}

def operand(mode, mem, pc):
    if mode in ('imp','acc'): return ''
    if mode=='imm': return f'#${mem[pc+1]:02x}'
    if mode=='zp': return f'${mem[pc+1]:02x}'
    if mode=='zpx': return f'${mem[pc+1]:02x},X'
    if mode=='zpy': return f'${mem[pc+1]:02x},Y'
    if mode=='abs': return f'${mem[pc+2]:02x}{mem[pc+1]:02x}'
    if mode=='absx': return f'${mem[pc+2]:02x}{mem[pc+1]:02x},X'
    if mode=='absy': return f'${mem[pc+2]:02x}{mem[pc+1]:02x},Y'
    if mode=='indx': return f'(${mem[pc+1]:02x},X)'
    if mode=='indy': return f'(${mem[pc+1]:02x}),Y'
    if mode=='izp': return f'(${mem[pc+1]:02x})'
    if mode=='absind': return f'(${mem[pc+2]:02x}{mem[pc+1]:02x})'
    if mode=='abxind': return f'(${mem[pc+2]:02x}{mem[pc+1]:02x},X)'
    if mode=='rel':
        t=mem[pc+1]; t=t-256 if t>=0x80 else t
        return f'${pc+2+t:04x}'
    return ''

def disassemble(mem, start, count, base=0):
    pc = start
    out = []
    while pc < start+count and pc < len(mem)-1:
        op = mem[pc]
        if op not in M:
            out.append(f'{base+pc:04x}: .byte ${op:02x}   ;?')
            pc += 1; continue
        mode, mn = M[op]
        ln = LEN[mode]
        if pc+ln > len(mem): break
        ops = operand(mode, mem, pc)
        raw = ' '.join(f'{mem[pc+i]:02x}' for i in range(1,ln))
        out.append(f'{base+pc:04x}: {mn:4s} {ops:12s} ; {mem[pc]:02x} {raw}')
        pc += ln
    return out

if __name__=='__main__':
    mem = open(sys.argv[1],'rb').read()
    start = int(sys.argv[2],16)
    count = int(sys.argv[3],16)
    for l in disassemble(mem, start, count):
        print(l)

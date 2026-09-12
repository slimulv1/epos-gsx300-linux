#!/usr/bin/env python3
"""EPOS GSX 300 (CX21988) firmware recursive disassembler.
W65C02S instruction set per WDC datasheet (Table 5-2).
Seeds: entrypoints; follows JSR/JMP/branch. Holes = data.
Usage: recdis.py <ram.bin> <seed> [seed...]
"""
import sys

OP = {
 0x00:("BRK",0),0x01:("ORA (,X)",1),0x04:("TSB ",1),0x05:("ORA ",1),0x06:("ASL ",1),
 0x07:("RMB0 ",1),0x08:("PHP",0),0x09:("ORA #",1),0x0a:("ASL A",0),0x0c:("TSB ",2),
 0x0d:("ORA ",2),0x0e:("ASL ",2),0x0f:("BBR0 ",2),0x10:("BPL ",1),0x11:("ORA (,Y)",1),
 0x12:("ORA (,zp)",1),0x14:("TRB ",1),0x15:("ORA ,X",1),0x16:("ASL ,X",1),0x17:("RMB1 ",1),
 0x18:("CLC",0),0x19:("ORA ,Y",2),0x1a:("INC A",0),0x1c:("TRB ",2),0x1d:("ORA ,X",2),
 0x1e:("ASL ,X",2),0x1f:("BBR1 ",2),
 0x20:("JSR ",2),0x21:("AND (,X)",1),0x24:("BIT ",1),0x25:("AND ",1),0x26:("ROL ",1),
 0x27:("RMB2 ",1),0x28:("PLP",0),0x29:("AND #",1),0x2a:("ROL A",0),0x2c:("BIT ",2),
 0x2d:("AND ",2),0x2e:("ROL ",2),0x2f:("BBR2 ",2),
 0x30:("BMI ",1),0x31:("AND (,Y)",1),0x32:("AND (,zp)",1),0x34:("BIT ,X",1),
 0x35:("AND ,X",1),0x36:("ROL ,X",1),0x37:("RMB3 ",1),0x38:("SEC",0),0x39:("AND ,Y",2),
 0x3a:("DEC A",0),0x3c:("BIT ,X",2),0x3d:("AND ,X",2),0x3e:("ROL ,X",2),0x3f:("BBR3 ",2),
 0x40:("RTI",0),0x41:("EOR (,X)",1),0x45:("EOR ",1),0x46:("LSR ",1),0x47:("RMB4 ",1),
 0x48:("PHA",0),0x49:("EOR #",1),0x4a:("LSR A",0),0x4c:("JMP ",2),0x4d:("EOR ",2),
 0x4e:("LSR ",2),0x4f:("BBR4 ",2),
 0x50:("BVC ",1),0x51:("EOR (,Y)",1),0x52:("EOR (,zp)",1),0x55:("EOR ,X",1),
 0x56:("LSR ,X",1),0x57:("RMB5 ",1),0x58:("CLI",0),0x59:("EOR ,Y",2),0x5a:("PHY",0),
 0x5d:("EOR ,X",2),0x5e:("LSR ,X",2),0x5f:("BBR5 ",2),
 0x60:("RTS",0),0x61:("ADC (,X)",1),0x64:("STZ ",1),0x65:("ADC ",1),0x66:("ROR ",1),
 0x67:("RMB6 ",1),0x68:("PLA",0),0x69:("ADC #",1),0x6a:("ROR A",0),0x6c:("JMP (",2),
 0x6d:("ADC ",2),0x6e:("ROR ",2),0x6f:("BBR6 ",2),
 0x70:("BVS ",1),0x71:("ADC (,Y)",1),0x72:("ADC (,zp)",1),0x74:("STZ ,X",1),
 0x75:("ADC ,X",1),0x76:("ROR ,X",1),0x77:("RMB7 ",1),0x78:("SEI",0),0x79:("ADC ,Y",2),
 0x7a:("PLY",0),0x7c:("JMP (,X)",2),0x7d:("ADC ,X",2),0x7e:("ROR ,X",2),0x7f:("BBR7 ",2),
 0x80:("BRA ",1),0x81:("STA (,X)",1),0x84:("STY ",1),0x85:("STA ",1),0x86:("STX ",1),
 0x87:("SMB0 ",1),0x88:("DEY",0),0x89:("BIT #",1),0x8a:("TXA",0),0x8c:("STY ",2),
 0x8d:("STA ",2),0x8e:("STX ",2),0x8f:("BBS0 ",2),
 0x90:("BCC ",1),0x91:("STA (,Y)",1),0x92:("STA (,zp)",1),0x94:("STY ,X",1),
 0x95:("STA ,X",1),0x96:("STX ,Y",1),0x97:("SMB1 ",1),0x98:("TYA",0),0x99:("STA ,Y",2),
 0x9a:("TXS",0),
 0x9c:("STZ ",2),0x9d:("STA ,X",2),0x9e:("STZ ,X",2),0x9f:("BBS1 ",2),
 0xa0:("LDY #",1),0xa1:("LDA (,X)",1),0xa2:("LDX #",1),0xa4:("LDY ",1),0xa5:("LDA ",1),
 0xa6:("LDX ",1),0xa7:("SMB2 ",1),0xa8:("TAY",0),0xa9:("LDA #",1),0xaa:("TAX",0),
 0xac:("LDY ",2),0xad:("LDA ",2),0xae:("LDX ",2),0xaf:("BBS2 ",2),
 0xb0:("BCS ",1),0xb1:("LDA (,Y)",1),0xb2:("LDA (,zp)",1),0xb4:("LDY ,X",1),
 0xb5:("LDA ,X",1),0xb6:("LDX ,Y",1),0xb7:("SMB3 ",1),0xb8:("CLV",0),0xb9:("LDA ,Y",2),
 0xba:("TSX",0),0xbc:("LDY ,X",2),0xbd:("LDA ,X",2),0xbe:("LDX ,Y",2),0xbf:("BBS3 ",2),
 0xc0:("CPY #",1),0xc1:("CMP (,X)",1),0xc4:("CPY ",1),0xc5:("CMP ",1),0xc6:("DEC ",1),
 0xc7:("SMB4 ",1),0xc8:("INY",0),0xc9:("CMP #",1),0xca:("DEX",0),0xcb:("WAI",0),
 0xcc:("CPY ",2),0xcd:("CMP ",2),0xce:("DEC ",2),0xcf:("BBS4 ",2),
 0xd0:("BNE ",1),0xd1:("CMP (,Y)",1),0xd2:("CMP (,zp)",1),0xd5:("CMP ,X",1),
 0xd6:("DEC ,X",1),0xd7:("SMB5 ",1),0xd8:("CLD",0),0xd9:("CMP ,Y",2),0xda:("PHX",0),
 0xdb:("STP",0),
 0xdd:("CMP ,X",2),0xde:("DEC ,X",2),0xdf:("BBS5 ",2),
 0xe0:("CPX #",1),0xe1:("SBC (,X)",1),0xe4:("CPX ",1),0xe5:("SBC ",1),0xe6:("INC ",1),
 0xe7:("SMB6 ",1),0xe8:("INX",0),0xe9:("SBC #",1),0xea:("NOP",0),0xec:("CPX ",2),
 0xed:("SBC ",2),0xee:("INC ",2),0xef:("BBS6 ",2),
 0xf0:("BEQ ",1),0xf1:("SBC (,Y)",1),0xf2:("SBC (,zp)",1),0xf5:("SBC ,X",1),
 0xf6:("INC ,X",1),0xf7:("SMB7 ",1),0xf8:("SED",0),0xf9:("SBC ,Y",2),0xfa:("PLX",0),
 0xfb:("NOP",0),
 0xfd:("SBC ,X",2),0xfe:("INC ,X",2),0xff:("BBS7 ",2),
}
# ---- aliases for branch/math ------  (keep short names for abs disasm)
branch_rel = {0x90,0xb0,0xf0,0xd0,0x30,0x10,0x50,0x70,0x80}

def main():
    data = open(sys.argv[1],'rb').read()
    if len(data) < 0x10000:
        data = data + b'\x00'*(0x10000-len(data))
    code = [False]*0x10000
    insn = {}
    queue = [int(x,16) for x in sys.argv[2:]]
    MAX = 0x10000
    jmp_abs = {0x4c}
    jsr = {0x20}
    while queue:
        pc = queue.pop(0)
        if pc >= MAX or insn.get(pc): continue
        while not insn.get(pc) and pc < MAX:
            if data[pc] == 0x00 and pc != 0x00 and (pc & 0xff00) != 0:
                break  # 0x00 padding in page >0 → likely data hole
            op = data[pc]
            z = OP.get(op)
            if z is None:
                insn[pc] = ([pc+1], f"{op:02x} !UNDEF", 1)
                code[pc] = True; pc += 1
                continue
            name, mode = z
            size = {0:1,1:2,2:3}[mode]
            if pc+size > MAX: break
            operand = data[pc+1:pc+size]
            # decode text
            if mode==0:
                txt = name
            elif mode==1:
                txt = f"{name}${operand[0]:02x}"
            else:
                if name.startswith(('BBR','BBS')):
                    off = operand[2] if len(operand)>2 else 0
                    if off >= 0x80: off -= 0x100
                    txt = f"{name}${operand[0]:02x} → ${pc+3+off:04x}"
                else:
                    word = operand[0] | ((operand[1]<<8) if len(operand)>1 else 0)
                    if name in ("JMP ","JSR "):
                        txt = f"JMP ${word:04x}" if name=="JMP " else f"JSR ${word:04x}"
                    elif name=="JMP (":
                        txt = f"JMP (${word:04x})"
                    else:
                        txt = f"{name}${word:04x}"
            nxt = [pc+size]
            word = 0
            if size > 1:
                word = operand[0] | ((operand[1]<<8) if len(operand)>1 else 0)
            if op in branch_rel:
                off = operand[0]
                if off >= 0x80: off -= 0x100
                nxt.append(pc+2+off)
            elif name.startswith(('BBR','BBS')):
                off = operand[2] if len(operand)>2 else 0
                if off >= 0x80: off -= 0x100
                nxt.append(pc+3+off)
            elif op in jmp_abs:
                nxt = [word]
            elif op == 0x6c:
                nxt = []  # JMP (abs) indirect - unresolved
            elif op == 0x7c:
                nxt = []  # JMP (abs,X) indexed indirect - unresolved
            elif op in jsr:
                nxt = [word, pc+size]
            insn[pc] = (nxt, txt, size)
            if op in branch_rel or name.startswith(('BBR','BBS')) or op in jmp_abs or op in jsr:
                for t in nxt:
                    if t != pc and (0x1400 <= t < MAX) and not insn.get(t):
                        queue.append(t)
            code[pc] = True
            pc += size
    i = 0
    while i < MAX:
        if insn.get(i):
            nxt, txt, size = insn[i]
            print(f"{i:04x}: {txt}")
            i += size
        else:
            i += 1

if __name__ == "__main__":
    main()
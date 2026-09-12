b001: BEQ $08
b003: CMP #$1b
b005: BEQ $04
b007: CMP #$03
b009: BNE $ee
b00b: RTS
b00c: JSR $b063
b00f: CMP #$05
b011: BCS $04
b013: SMB4 $99
b015: LDA #$05
b017: SEC
b018: SBC #$05
b01a: STA $0262
b01d: RTS
b01e: JSR $b063
b021: CLC
b022: ADC $0265
b025: STA $0265
b028: RTS
b029: JSR $b063
b02c: CLC
b02d: ADC $0264
b030: STA $0264
b033: BCC $03
b035: INC $0265
b038: RTS
b039: LDX #$00
b03b: LDY $0262
b03e: BEQ $12
b040: JSR $b063
b043: BBS4 $99 → $b046
b046: STA ,X$0242
b049: INX
b04a: DEY
b04b: BNE $f3
b04d: LDA #$2e
b04f: JSR $a5a5
b052: RTS
b053: JSR $b063
b056: BBS4 $99 → $b059
b059: LDA $0263
b05c: CMP #$ff
b05e: BEQ $02
b060: SMB4 $99
b062: RTS
b063: JSR $b07d
b066: ASL A
b067: ASL A
b068: ASL A
b069: ASL A
b06a: STA $0266
b06d: JSR $b07d
b070: ORA $0266
b073: PHA
b074: CLC
b075: ADC $0263
b078: STA $0263
b07b: PLA
b07c: RTS
b07d: JSR $b1e8
b080: CMP #$41
b082: BCC $02
b084: SBC #$08
b086: SBC #$2f
b088: RTS
b089: BBS5 $97 → $b08c
b08c: LDX #$00
b08e: LDY $0262
b091: BEQ $13
b093: LDA $0265
b096: STA $41
b098: LDA $0264
b09b: STA $40
b09d: LDA ,X$0242
b0a0: STA (,Y)$40
b0a2: INX
b0a3: DEY
b0a4: BNE $f7
b0a6: RTS
b0a7: RMB4 $99
b0a9: LDA $0265
b0ac: STA $43
b0ae: LDA $0264
b0b1: STA $42
b0b3: LDX #$00
b0b5: LDA $0262
b0b8: BEQ $ec
b0ba: LDA $0265
b0bd: STA $43
b0bf: LDA $0264
b0c2: STA $42
b0c4: LDA $98
b0c6: AND #$02
b0c8: STA $49
b0ca: LDA ,X$0242
b0cd: JSR $acbf
b0d0: INX
b0d1: CPX $0262
b0d4: BNE $f4
b0d6: BBR0 $9a → $b0d9
b0d9: JSR $acf0
b0dc: LDA $98
b0de: AND #$fd
b0e0: ORA $49
b0e2: STA $98
b0e4: LDX #$00
b0e6: LDA $0265
b0e9: STA $41
b0eb: LDA $0264
b0ee: STA $40
b0f0: RMB5 $98
b0f2: JSR $b106
b0f5: CMP ,X$0242
b0f8: BEQ $02
b0fa: SMB4 $99
b0fc: INX
b0fd: DEC $0262
b100: BNE $f0
b102: JSR $ae56
b105: RTS
b106: BBS5 $98 → $b109
b109: SMB5 $98
b10b: PHX
b10c: LDX #$00
b10e: JSR $ac53
b111: PLX
b112: BRA $03
b114: JSR $ad3a
b117: PHA
b118: INC $40
b11a: BNE $16
b11c: INC $41
b11e: BNE $04
b120: SMB1 $98
b122: BRA $09
b124: BBS2 $98 → $b127
b127: LDY #$80
b129: CPY $41
b12b: BNE $05
b12d: JSR $ae4c
b130: PLA
b131: RTS
b132: JSR $ae60
b135: PLA
b136: RTS
b137: LDX #$00
b139: RMB5 $98
b13b: JSR $b140
b13e: BRA $fb
b140: CPX #$16
b142: BEQ $03
b144: JSR $eba7
b147: JMP (,X)$b14a
b14a: 62 !UNDEF
b14b: LDA (,Y)$82
b14d: LDA (,Y)$89
b14f: LDA (,Y)$90
b151: LDA (,Y)$94
b153: LDA (,Y)$9d
b155: LDA (,Y)$82
b157: LDA (,Y)$89
b159: LDA (,Y)$a1
b15b: LDA (,Y)$90
b15d: LDA (,Y)$94
b15f: LDA (,Y)$a8
b161: LDA (,Y)$48
b163: PHA
b164: LDA #$00
b166: STA $47
b168: STA $46
b16a: STA $45
b16c: STA $44
b16e: STA $43
b170: STA $42
b172: RMB6 $99
b174: RMB7 $99
b176: PLA
b177: AND #$40
b179: BEQ $02
b17b: SMB6 $99
b17d: PLA
b17e: SMB7 $99
b180: BRA $23
b182: STA $46
b184: STA $0264
b187: BRA $1c
b189: STA $47
b18b: STA $0265
b18e: BRA $15
b190: STA $42
b192: BRA $11
b194: STA $43
b196: BBR7 $99 → $b199
b199: LDX #$14
b19b: BRA $08
b19d: STA $45
b19f: BRA $04
b1a1: STA $44
b1a3: BRA $00
b1a5: INX
b1a6: INX
b1a7: RTS
b1a8: JSR $a851
b1ab: JSR $eba7
b1ae: STA (,X)$46
b1b0: JSR $a858
b1b3: LDA $47
b1b5: CMP $43
b1b7: BNE $04
b1b9: LDA $46
b1bb: CMP $42
b1bd: BNE $ec
b1bf: BBS6 $99 → $b1c2
b1c2: LDX #$00
b1c4: RTS
b1c5: LDA $0265
b1c8: STA $47
b1ca: LDA $0264
b1cd: STA $46
b1cf: JSR $b1d4
b1d2: BRA $ee
b1d4: JMP ($0046)
b1d7: 22 !UNDEF
b1d8: LDY #$c0
b1da: JSR $b1e2
b1dd: DEY
b1de: BNE $fa
b1e0: AND (,zp)$60
b1e2: LDX #$ff
b1e4: DEX
b1e5: BNE $fd
b1e7: RTS
b1e8: BRA $00
b1ea: PHX
b1eb: LDX $0240
b1ee: CPX $0241
b1f1: BEQ $f8
b1f3: JSR $b1fd
b1f6: BBR0 $99 → $b1f9
b1f9: RMB1 $01
b1fb: PLX
b1fc: RTS
b1fd: LDX $0240
b200: LDA ,X$0200
b203: PHA
b204: INX
b205: TXA
b206: AND #$3f
b208: STA $0240
b20b: LDA $0241
b20e: SEC
b20f: SBC $0240
b212: AND #$3f
b214: CMP #$10
b216: BCS $02
b218: SMB0 $99
b21a: PLA
b21b: RTS
b21c: JSR $b225
b21f: BBS0 $99 → $b222
b222: SMB1 $01
b224: RTS
b225: PHX
b226: LDX $0241
b229: STA ,X$0200
b22c: INX
b22d: TXA
b22e: AND #$3f
b230: STA $0241
b233: SEC
b234: SBC $0240
b237: AND #$3f
b239: CMP #$20
b23b: BCC $02
b23d: RMB0 $99
b23f: PLX
b240: RTS
b241: STP
b242: 22 !UNDEF
b243: ab !UNDEF
b244: ASL A
b245: TAX
b246: JSR $b24b
b249: AND (,zp)$03
b24b: JMP (,X)$b24e
b24e: ROR ,X$82b2
b251: LDA (,zp)$af
b253: LDA (,zp)$b3
b255: LDA (,zp)$e8
b257: LDA (,zp)$ec
b259: LDA (,zp)$86
b25b: LDA (,zp)$8a
b25d: LDA (,zp)$8e
b25f: LDA (,zp)$92
b261: LDA (,zp)$96
b263: LDA (,zp)$9a
b265: LDA (,zp)$b7
b267: LDA (,zp)$bb
b269: LDA (,zp)$bf
b26b: LDA (,zp)$c3
b26d: LDA (,zp)$c7
b26f: LDA (,zp)$cb
b271: LDA (,zp)$f0
b273: LDA (,zp)$f4
b275: LDA (,zp)$f8
b277: LDA (,zp)$fc
b279: LDA (,zp)$00
b27b: b3 !UNDEF
b27c: TSB $b3
b27e: LDX #$00
b280: BRA $1c
b282: LDX #$02
b284: BRA $18
b286: LDX #$08
b288: BRA $14
b28a: LDX #$0a
b28c: BRA $10
b28e: LDX #$10
b290: BRA $0c
b292: LDX #$12
b294: BRA $08
b296: LDX #$18
b298: BRA $04
b29a: LDX #$1a
b29c: BRA $00
b29e: ab !UNDEF
b29f: STA ,X$0b00
b2a2: ab !UNDEF
b2a3: STA ,X$0b01
b2a6: ab !UNDEF
b2a7: STA ,X$0b04
b2aa: ab !UNDEF
b2ab: STA ,X$0b05
b2ae: RTS
b2af: LDX #$00
b2b1: BRA $1c
b2b3: LDX #$02
b2b5: BRA $18
b2b7: LDX #$08
b2b9: BRA $14
b2bb: LDX #$0a
b2bd: BRA $10
b2bf: LDX #$10
b2c1: BRA $0c
b2c3: LDX #$12
b2c5: BRA $08
b2c7: LDX #$18
b2c9: BRA $04
b2cb: LDX #$1a
b2cd: BRA $00
b2cf: LDA ,X$0b00
b2d2: STA ,X$027b
b2d5: LDA ,X$0b01
b2d8: STA ,X$027c
b2db: LDA ,X$0b04
b2de: STA ,X$027f
b2e1: LDA ,X$0b05
b2e4: STA ,X$0280
b2e7: RTS
b2e8: LDX #$00
b2ea: BRA $1c
b2ec: LDX #$02
b2ee: BRA $18
b2f0: LDX #$08
b2f2: BRA $14
b2f4: LDX #$0a
b2f6: BRA $10
b2f8: LDX #$10
b2fa: BRA $0c
b2fc: LDX #$12
b2fe: BRA $08
b300: LDX #$18
b302: BRA $04
b304: LDX #$1a
b306: BRA $00
b308: LDA ,X$027b
b30b: STA ,X$0b00
b30e: LDA ,X$027c
b311: STA ,X$0b01
b314: LDA ,X$027f
b317: STA ,X$0b04
b31a: LDA ,X$0280
b31d: STA ,X$0b05
b320: RTS
b321: BBS7 $98 → $b324
b324: JSR $ae2b
b327: LDX #$04
b329: PLA
b32a: DEX
b32b: BNE $fc
b32d: RTS
b32e: SBC (,zp)$00
b330: TSB $80
b332: TRB $00f2
b335: TSB $40
b337: ORA (,X)$60
b339: c2 !UNDEF
b33a: BPL $01
b33c: ORA $14a2
b33f: JSR $a4c5
b342: LDX #$1f
b344: LDA #$00
b346: STA ,X$0b00
b349: DEX
b34a: BPL $fa
b34c: JMP ($fffe)
b34f: JSR $13f0
b352: LDA $02fc
b355: STA $40
b357: LDA $02fd
b35a: STA $41
b35c: LDX #$00
b35e: SBC (,zp)$00
b360: TSB $01
b362: ROL $e2
b7c0: JSR $13ed
b7c3: JSR $b862
b7c6: JSR $b8a7
b7c9: LDA (,zp)$7d
b7cb: RTI
b7cc: LDA (,zp)$ec
b7ce: EOR (,X)$20
b7d0: f4 !UNDEF
b7d1: LDA $8d
b7d3: 02 !UNDEF
b7d4: BPL $20
b7d6: LSR A
b7d7: TAY
b7d8: JSR $a5f4
b7db: STA $1001
b7de: JSR $a84a
b7e1: JSR $a5f4
b7e4: STA $1004
b7e7: JSR $a84a
b7ea: JSR $a5f4
b7ed: STA $1003
b7f0: LDA #$0f
b7f2: STA $1006
b7f5: LDA $66
b7f7: STA $126a
b7fa: STA $126c
b7fd: LDA $67
b7ff: STA $126b

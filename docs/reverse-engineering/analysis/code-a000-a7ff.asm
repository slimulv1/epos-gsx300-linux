a000: JSR $a090
a003: SBC (,zp)$7d
a005: BBR0 $80 → $a008
a008: SEI
a009: LDX #$ff
a00b: TXS
a00c: c2 !UNDEF
a00d: ORA (,X)$e5
a00f: TSB $a9
a06b: LDA (,zp)$b4
a06d: RTI
a06e: LDA (,zp)$13
a070: EOR (,X)$a9
a072: RTS
a073: STA (,X)$40
a075: JSR $a84a
a078: LDA $41
a07a: CMP #$14
a07c: BNE $04
a07e: LDA $40
a080: CMP #$8f
a082: BNE $ed
a084: LDA #$00
a086: STA $148f
a089: STA $1490
a08c: STA $1491
a08f: RTS
a090: SEI
a091: TSX
a092: INX
a093: LDA ,X$0100
a096: JSR $a5c9
a099: LDA #$20
a09b: JSR $a5a5
a09e: INX
a09f: BNE $f2
a0a1: CLI
a0a2: RTS
a0a3: 22 !UNDEF
a0a4: RMB5 $0b
a0a6: AND (,zp)$40
a0a8: 22 !UNDEF
a0a9: e2 !UNDEF
a0aa: PHP
a0ab: ORA $1a01
a0ae: SBC (,zp)$06
a0b0: ASL $1501
a0b3: SBC (,zp)$06
a0b5: ASL $1080
a0b8: LDA $0e04
a0bb: BNE $0b
a0bd: LDA #$01
a0bf: STA $0d08
a0c2: CMP (,zp)$01
a0c4: ASL $0e
a0c6: AND (,zp)$40
a0c8: SBC (,zp)$54
a0ca: TSB $80
a0cc: 03 !UNDEF
a0cd: JMP $a178
a0d0: LDA $0457
a0d3: c2 !UNDEF
a0d4: RTI
a0d5: 54 !UNDEF
a0d6: TSB $58
a0d8: LDA $a9
a0da: BEQ $02
a0dc: DEC $a9
a0de: LDA $a8
a0e0: BEQ $02
a0e2: DEC $a8
a0e4: BBR0 $4f → $a0e7
a0e7: JSR $1459
a0ea: LDA $d8
a0ec: BEQ $0b
a0ee: CMP #$44
a0f0: BEQ $07
a0f2: LDA $00
a0f4: AND #$44
a0f6: PHA
a0f7: BRA $25
a0f9: LDA $00
a0fb: AND #$44
a0fd: PHA
a0fe: EOR $d8
a100: BEQ $1c
a102: CMP #$44
a104: BEQ $18
a106: CMP #$40
a108: BEQ $0a
a10a: LDA $12d2
a10d: BEQ $0f
a10f: INC $12d2
a112: BRA $0a
a114: LDA $12d2
a117: CMP #$b1
a119: BEQ $03
a11b: DEC $12d2
a11e: PLA
a11f: STA $d8
a121: BBR3 $93 → $a124
a124: JSR $123e
a127: CMP (,zp)$40
a129: 54 !UNDEF
a12a: TSB $32
a12c: RTI
a12d: 22 !UNDEF
a12e: RMB3 $0b
a130: BBR0 $3c → $a133
a133: WAI
a134: LDA $40
a136: LDX $41
a138: LDY $42
a13a: 22 !UNDEF
a13b: LDA $43
a13d: LDX $44
a13f: LDY $45
a141: 22 !UNDEF
a142: LDA $46
a144: LDX $47
a146: 22 !UNDEF
a147: 23 !UNDEF
a148: LDA $38
a14a: BBR7 $93 → $a14d
a14d: JSR $1453
a150: JSR $1238
a153: 33 !UNDEF
a154: AND (,zp)$85
a156: LSR $86
a158: RMB4 $32
a15a: STA $43
a15c: STX $44
a15e: STY $45
a160: AND (,zp)$85
a162: RTI
a163: STX $41
a165: STY $42
a167: STP
a168: AND (,zp)$40
a16a: JSR $13c3
a16d: BBR1 $39 → $a170
a170: BBR5 $3c → $a173
a173: 22 !UNDEF
a174: RMB1 $39
a176: AND (,zp)$40
a178: JSR $be3b
a17b: JSR $bf1e
a17e: LDA $137d
a181: JSR $a186
a184: AND (,zp)$40
a186: ASL A
a187: TAX
a188: JMP (,X)$a18b
a18b: STA ,Y$99a1
a18e: LDA (,X)$9d
a190: LDA (,X)$9d
a192: LDA (,X)$9d
a194: LDA (,X)$9d
a196: LDA (,X)$9d
a198: LDA (,X)$ad
a19a: 02 !UNDEF
a19b: ORA $a560
a19e: RTI
a19f: LDX $41
a1a1: LDY $42
a1a3: 22 !UNDEF
a1a4: LDA $43
a1a6: LDX $44
a1a8: LDY $45
a1aa: 22 !UNDEF
a1ab: LDA $46
a1ad: LDX $47
a1af: 22 !UNDEF
a1b0: JSR $bf4d
a1b3: JSR $c110
a1b6: AND (,zp)$85
a1b8: LSR $86
a1ba: RMB4 $32
a1bc: STA $43
a1be: STX $44
a1c0: STY $45
a1c2: AND (,zp)$85
a1c4: RTI
a1c5: STX $41
a1c7: STY $42
a1c9: JSR $be97
a1cc: JSR $bed8
a1cf: JMP $bec6
a1d2: LDA #$03
a1d4: STA $123f
a1d7: LDA #$a2
a1d9: STA $1240
a1dc: BBS0 $94 → $a1df
a1df: LDA $ab
a1e1: BNE $0a
a1e3: LDA $ac
a1e5: BNE $04
a1e7: SMB0 $94
a1e9: BRA $04
a1eb: DEC $ac
a1ed: DEC $ab
a1ef: BBS2 $95 → $a1f2
a1f2: LDA $b5
a1f4: BNE $0a
a1f6: LDA $b6
a1f8: BNE $04
a1fa: SMB2 $95
a1fc: BRA $04
a1fe: DEC $b6
a200: DEC $b5
a202: RTS
a203: LDA #$34
a205: STA $123f
a208: LDA #$a2
a20a: STA $1240
a20d: BBS1 $94 → $a210
a210: LDA $ad
a212: BNE $0a
a214: LDA $ae
a216: BNE $04
a218: SMB1 $94
a21a: BRA $04
a21c: DEC $ae
a21e: DEC $ad
a220: BBS3 $95 → $a223
a223: LDA $b7
a225: BNE $0a
a227: LDA $b8
a229: BNE $04
a22b: SMB3 $95
a22d: BRA $04
a22f: DEC $b8
a231: DEC $b7
a233: RTS
a234: LDA #$65
a236: STA $123f
a239: LDA #$a2
a23b: STA $1240
a23e: BBS2 $94 → $a241
a241: LDA $af
a243: BNE $0a
a245: LDA $b0
a247: BNE $04
a249: SMB2 $94
a24b: BRA $04
a24d: DEC $b0
a24f: DEC $af
a251: BBS4 $95 → $a254
a254: LDA $b9
a256: BNE $0a
a258: LDA $ba
a25a: BNE $04
a25c: SMB4 $95
a25e: BRA $04
a260: DEC $ba
a262: DEC $b9
a264: RTS
a265: LDA #$96
a267: STA $123f
a26a: LDA #$a2
a26c: STA $1240
a26f: BBS3 $94 → $a272
a272: LDA $b1
a274: BNE $0a
a276: LDA $b2
a278: BNE $04
a27a: SMB3 $94
a27c: BRA $04
a27e: DEC $b2
a280: DEC $b1
a282: BBS5 $95 → $a285
a285: LDA $bb
a287: BNE $0a
a289: LDA $bc
a28b: BNE $04
a28d: SMB5 $95
a28f: BRA $04
a291: DEC $bc
a293: DEC $bb
a295: RTS
a296: LDA #$c7
a298: STA $123f
a29b: LDA #$a2
a29d: STA $1240
a2a0: BBS4 $94 → $a2a3
a2a3: LDA $b3
a2a5: BNE $0a
a2a7: LDA $b4
a2a9: BNE $04
a2ab: SMB4 $94
a2ad: BRA $04
a2af: DEC $b4
a2b1: DEC $b3
a2b3: BBS6 $95 → $a2b6
a2b6: LDA $bd
a2b8: BNE $0a
a2ba: LDA $be
a2bc: BNE $04
a2be: SMB6 $95
a2c0: BRA $04
a2c2: DEC $be
a2c4: DEC $bd
a2c6: RTS
a2c7: LDA #$e5
a2c9: STA $123f
a2cc: LDA #$a2
a2ce: STA $1240
a2d1: BBS7 $95 → $a2d4
a2d4: LDA $bf
a2d6: BNE $0a
a2d8: LDA $c0
a2da: BNE $04
a2dc: SMB7 $95
a2de: BRA $04
a2e0: DEC $c0
a2e2: DEC $bf
a2e4: RTS
a2e5: LDA #$03
a2e7: STA $123f
a2ea: LDA #$a3
a2ec: STA $1240
a2ef: BBS0 $96 → $a2f2
a2f2: LDA $c1
a2f4: BNE $0a
a2f6: LDA $c2
a2f8: BNE $04
a2fa: SMB0 $96
a2fc: BRA $04
a2fe: DEC $c2
a300: DEC $c1
a302: RTS
a303: LDA #$21
a305: STA $123f
a308: LDA #$a3
a30a: STA $1240
a30d: BBS1 $96 → $a310
a310: LDA $c3
a312: BNE $0a
a314: LDA $c4
a316: BNE $04
a318: SMB1 $96
a31a: BRA $04
a31c: DEC $c4
a31e: DEC $c3
a320: RTS
a321: LDA #$3f
a323: STA $123f
a326: LDA #$a3
a328: STA $1240
a32b: BBS2 $96 → $a32e
a32e: LDA $c5
a330: BNE $0a
a332: LDA $c6
a334: BNE $04
a336: SMB2 $96
a338: BRA $04
a33a: DEC $c6
a33c: DEC $c5
a33e: RTS
a33f: LDA #$d2
a341: STA $123f
a344: LDA #$a1
a346: STA $1240
a349: BBR1 $99 → $a34c
a34c: RTS
a34d: SMB1 $99
a34f: CMP (,zp)$40
a351: 54 !UNDEF
a352: TSB $58
a354: BBR4 $93 → $a357
a357: JSR $13c6
a35a: JSR $1241
a35d: LDA $aa
a35f: BEQ $02
a361: DEC $aa
a363: JSR $e9c3
a366: SEI
a367: RMB1 $99
a369: RTS
a36a: LDA $1009
a36d: BEQ $08
a36f: LDA $d1
a371: CMP #$ff
a373: BEQ $02
a375: INC $d1
a377: LDA #$84
a379: STA $1242
a37c: LDA #$a3
a37e: STA $1243
a381: SMB1 $9d
a383: RTS
a384: LDA #$a2
a386: STA $1242
a389: LDA #$a3
a38b: STA $1243
a38e: BBS5 $94 → $a391
a391: LDA $c7
a393: BNE $0a
a395: LDA $c8
a397: BNE $04
a399: SMB5 $94
a39b: BRA $04
a39d: DEC $c8
a39f: DEC $c7
a3a1: RTS
a3a2: LDA #$c2
a3a4: STA $1242
a3a7: LDA #$a3
a3a9: STA $1243
a3ac: BBS6 $94 → $a3af
a3af: LDA $c9
a3b1: BNE $0a
a3b3: LDA $ca
a3b5: BNE $04
a3b7: SMB6 $94
a3b9: BRA $04
a3bb: DEC $ca
a3bd: DEC $c9
a3bf: SMB1 $9d
a3c1: RTS
a3c2: LDA #$e0
a3c4: STA $1242
a3c7: LDA #$a3
a3c9: STA $1243
a3cc: BBS7 $94 → $a3cf
a3cf: LDA $cb
a3d1: BNE $0a
a3d3: LDA $cc
a3d5: BNE $04
a3d7: SMB7 $94
a3d9: BRA $04
a3db: DEC $cc
a3dd: DEC $cb
a3df: RTS
a3e0: LDA #$ed
a3e2: STA $1242
a3e5: LDA #$a3
a3e7: STA $1243
a3ea: SMB1 $9d
a3ec: RTS
a3ed: LDA #$f8
a3ef: STA $1242
a3f2: LDA #$a3
a3f4: STA $1243
a3f7: RTS
a3f8: LDA #$05
a3fa: STA $1242
a3fd: LDA #$a4
a3ff: STA $1243
a402: SMB1 $9d
a404: RTS
a405: LDA #$19
a407: STA $1242
a40a: LDA #$a4
a40c: STA $1243
a40f: BBR5 $93 → $a412
a412: JSR $13c9
a415: JSR $1244
a418: RTS
a419: LDA #$26
a41b: STA $1242
a41e: LDA #$a4
a420: STA $1243
a423: SMB1 $9d
a425: RTS
a426: LDA #$6a
a428: STA $1242
a42b: LDA #$a3
a42d: STA $1243
a430: RTS
a431: LDA #$4f
a433: STA $1245
a436: LDA #$a4
a438: STA $1246
a43b: BBS0 $95 → $a43e
a43e: LDA $cd
a440: BNE $0a
a442: LDA $ce
a444: BNE $04
a446: SMB0 $95
a448: BRA $04
a44a: DEC $ce
a44c: DEC $cd
a44e: RTS
a44f: LDA #$6d
a451: STA $1245
a454: LDA #$a4
a456: STA $1246
a459: BBS1 $95 → $a45c
a45c: LDA $cf
a45e: BNE $0a
a460: LDA $d0
a462: BNE $04
a464: SMB1 $95
a466: BRA $04
a468: DEC $d0
a46a: DEC $cf
a46c: RTS
a46d: LDA #$78
a46f: STA $1245
a472: LDA #$a4
a474: STA $1246
a477: RTS
a478: LDA #$83
a47a: STA $1245
a47d: LDA #$a4
a47f: STA $1246
a482: RTS
a483: LDA #$8e
a485: STA $1245
a488: LDA #$a4
a48a: STA $1246
a48d: RTS
a48e: LDA #$99
a490: STA $1245
a493: LDA #$a4
a495: STA $1246
a498: RTS
a499: LDA #$a4
a49b: STA $1245
a49e: LDA #$a4
a4a0: STA $1246
a4a3: RTS
a4a4: LDA #$af
a4a6: STA $1245
a4a9: LDA #$a4
a4ab: STA $1246
a4ae: RTS
a4af: LDA #$ba
a4b1: STA $1245
a4b4: LDA #$a4
a4b6: STA $1246
a4b9: RTS
a4ba: LDA #$31
a4bc: STA $1245
a4bf: LDA #$a4
a4c1: STA $1246
a4c4: RTS
a4c5: CPX #$00
a4c7: BEQ $06
a4c9: JSR $a4d0
a4cc: DEX
a4cd: BNE $fa
a4cf: RTS
a4d0: PHX
a4d1: LDX #$64
a4d3: JSR $a4f7
a4d6: PLX
a4d7: RTS
a4d8: CPX #$00
a4da: BEQ $06
a4dc: JSR $a4e3
a4df: DEX
a4e0: BNE $fa
a4e2: RTS
a4e3: PHX
a4e4: LDX #$0a
a4e6: JSR $a4f7
a4e9: PLX
a4ea: RTS
a4eb: CPX #$00
a4ed: BEQ $07
a4ef: CPX #$01
a4f1: BEQ $1f
a4f3: JSR $a4f7
a4f6: RTS
a4f7: 22 !UNDEF
a4f8: PHP
a4f9: TSX
a4fa: LDA ,X$0101
a4fd: PLP
a4fe: AND #$04
a500: BNE $05
a502: SBC (,zp)$54
a504: TSB $40
a506: 03 !UNDEF
a507: AND (,zp)$80
a509: ASL A
a50a: AND (,zp)$86
a50c: LDA #$a6
a50e: LDA #$d0
a510: fc !UNDEF
a511: RTS
a512: LDX #$01
a514: PHY
a515: LDY #$0a
a517: JSR $a51f
a51a: DEY
a51b: BNE $fa
a51d: PLY
a51e: RTS
a51f: PHX
a520: PHY
a521: LDY #$9b
a523: BBR3 $09 → $a526
a526: LDY #$4d
a528: JSR $a533
a52b: BNE $fb
a52d: DEX
a52e: BNE $f1
a530: PLY
a531: PLX
a532: RTS
a533: WAI
a534: STP
a535: NOP
a536: NOP
a537: NOP
a538: DEY
a539: RTS
a53a: PHX
a53b: PHY
a53c: LDY #$14
a53e: JSR $a549
a541: BNE $fb
a543: DEX
a544: BNE $f6
a546: PLY
a547: PLX
a548: RTS
a549: PHA
a54a: NOP
a54b: PLA
a54c: DEY
a54d: RTS
a54e: RTS
a54f: 22 !UNDEF
a550: RMB7 $0b
a552: STA $3c
a554: LDA $0452
a557: AND (,zp)$40
a559: 22 !UNDEF
a55a: RMB6 $0b
a55c: RMB5 $0b
a55e: LDA $044d
a561: ORA #$01
a563: STA $08
a565: AND (,zp)$40
a567: LDA ,X$ebba
a56a: STA $0459
a56d: LDA ,X$ebbb
a570: STA $045a
a573: LDA ,X$ebba
a576: STA $045d
a579: LDA ,X$ebbb
a57c: STA $045e
a57f: LDA (,zp)$0f
a581: ROL ,X$0fb2
a584: BBR3 $a9 → $a587
a587: STA $0458
a58a: STA $045c
a58d: RTS
a58e: SMB0 $39
a590: RTS
a591: RMB0 $39
a593: RTS
a594: SEC
a595: LDA (,zp)$00
a597: EOR (,X)$0f
a599: BIT ,X$2005
a59c: LDA (,X)$a5
a59e: STA $41
a5a0: RTS
a5a1: LDA $38
a5a3: CLC
a5a4: RTS
a5a5: BBS5 $99 → $a5a8
a5a8: BBR5 $3c → $a5ab
a5ab: STA $38
a5ad: RTS
a5ae: BBS5 $99 → $a5b1
a5b1: STA $38
a5b3: RTS
a5b4: BBR7 $3a → $a5b7
a5b7: BBR5 $3c → $a5ba
a5ba: BBR6 $3c → $a5bd
a5bd: RTS
a5be: LDA #$0d
a5c0: BRA $e3
a5c2: JSR $a5be
a5c5: LDA #$0a
a5c7: BRA $dc
a5c9: PHA
a5ca: LSR A
a5cb: LSR A
a5cc: LSR A
a5cd: LSR A
a5ce: JSR $a5d4
a5d1: PLA
a5d2: AND #$0f
a5d4: JSR $a5da
a5d7: JMP $a5a5
a5da: CMP #$0a
a5dc: BCC $02
a5de: ADC #$06
a5e0: ADC #$30
a5e2: RTS
a5e3: CMP #$61
a5e5: BCC $04
a5e7: SBC #$28
a5e9: BRA $06
a5eb: CMP #$41
a5ed: BCC $02
a5ef: SBC #$08
a5f1: SBC #$2f
a5f3: RTS
a5f4: LDA (,X)$40
a5f6: JSR $a5e3
a5f9: ASL A
a5fa: ASL A
a5fb: ASL A
a5fc: ASL A
a5fd: STA $42
a5ff: JSR $a84a
a602: LDA (,X)$40
a604: JSR $a5e3
a607: JSR $a84a
a60a: ORA $42
a60c: RTS
a60d: LDA (,zp)$c0
a791: CMP $13b3
a794: BNE $01
a796: RTS
a797: STA $13b3
a79a: PHA
a79b: ASL A
a79c: ASL A
a79d: ASL A
a79e: STA $4b
a7a0: LDA $09
a7a2: AND #$c7
a7a4: ORA $4b
a7a6: STA $09
a7a8: PLA
a7a9: ASL A
a7aa: TAX
a7ab: LDA #$00
a7ad: STA $0480
a7b0: STA $0481
a7b3: JSR $a567
a7b6: LDA (,zp)$80
a7b8: AND ,X$03b2
a7bb: 3b !UNDEF
a7bc: LDA (,zp)$c0
a7be: DEC A
a7bf: LDA $38
a7c1: LDA #$00
a7c3: STA $0454
a7c6: LDA ,X$ebb0
a7c9: STA $0455
a7cc: LDA ,X$ebb1
a7cf: STA $0457
a7d2: CMP (,zp)$40
a7d4: 54 !UNDEF
a7d5: TSB $60
a7d7: BBR0 $98 → $a7da
a7da: LDA $1265
a7dd: CMP #$ff
a7df: BEQ $66
a7e1: LDA (,zp)$14
a7e3: RTI
a7e4: LDA (,zp)$00
a7e6: EOR (,X)$20
a7e8: SMB2 $ae
a7ea: CMP #$50
a7ec: BEQ $1a
a7ee: LDA (,zp)$71
a7f0: RTI
a7f1: LDA (,zp)$01
a7f3: EOR (,X)$20
a7f5: SMB2 $ae
a7f7: CMP #$50
a7f9: BEQ $0d
a7fb: LDA (,zp)$80
a7fd: RTI
a7fe: LDA (,zp)$01

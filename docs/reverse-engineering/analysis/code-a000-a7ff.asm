a000: JSR  $a090        ; 20 90 a0
a003: SBC  ($7d)        ; f2 7d
a005: .byte $0f   ;?
a006: BRA  $a003        ; 80 fb
a008: .byte $78   ;?
a009: LDX  #$ff         ; a2 ff
a00b: TXS               ; 9a 
a00c: .byte $c2   ;?
a00d: ORA  ($e5,X)      ; 01 e5
a00f: TSB  $a9          ; 04 a9
a011: BRK               ; 00 
a012: STA  $04e0,X      ; 8d e0 04
a015: .byte $22   ;?
a016: AND  ($d2)        ; 32 d2
a018: .byte $02   ;?
a019: CPX  #$04         ; e0 04
a01b: JSR  $a63a        ; 20 3a a6
a01e: CMP  ($01)        ; d2 01
a020: CPX  #$04         ; e0 04
a022: LDA  #$38         ; a9 38
a024: STA  $04e6,X      ; 8d e6 04
a027: CMP  ($01)        ; d2 01
a029: SBC  $04          ; e5 04
a02b: LDA  #$00         ; a9 00
a02d: STA  $1000,X      ; 8d 00 10
a030: JSR  $a60d        ; 20 0d a6
a033: JSR  $a06b        ; 20 6b a0
a036: JSR  $a6a8        ; 20 a8 a6
a039: .byte $a7   ;?
a03a: .byte $99   ;?
a03b: LDA  #$ff         ; a9 ff
a03d: STA  $13b3,X      ; 8d b3 13
a040: LDA  #$01         ; a9 01
a042: JSR  $a791        ; 20 91 a7
a045: JSR  $ac04        ; 20 04 ac
a048: JSR  $a981        ; 20 81 a9
a04b: BCS  $a05d        ; b0 10
a04d: JSR  $aa41        ; 20 41 aa
a050: JSR  $aead        ; 20 ad ae
a053: JSR  $a9ce        ; 20 ce a9
a056: BCC  $a05d        ; 90 05
a058: .byte $c7   ;?
a059: TYA               ; 98 
a05a: JMP  ($a9ed)      ; 4c ed a9
a05d: JSR  $a7d7        ; 20 d7 a7
a060: JSR  $e9c3        ; 20 c3 e9
a063: CMP  ($04)        ; d2 04
a065: .byte $78   ;?
a066: TSB  $4c          ; 04 4c
a068: RTS               ; 60 
a069: TAY               ; a8 
a06a: RTS               ; 60 
a06b: LDA  ($b4)        ; b2 b4
a06d: RTI               ; 40 
a06e: LDA  ($13)        ; b2 13
a070: EOR  ($a9,X)      ; 41 a9
a072: RTS               ; 60 
a073: STA  ($40,X)      ; 81 40
a075: JSR  $a84a        ; 20 4a a8
a078: LDA  $41          ; a5 41
a07a: CMP  #$14         ; c9 14
a07c: BNE  $a082        ; d0 04
a07e: LDA  $40          ; a5 40
a080: CMP  #$8f         ; c9 8f
a082: BNE  $a071        ; d0 ed
a084: LDA  #$00         ; a9 00
a086: STA  $148f,X      ; 8d 8f 14
a089: STA  $1490,X      ; 8d 90 14
a08c: STA  $1491,X      ; 8d 91 14
a08f: RTS               ; 60 
a090: .byte $78   ;?
a091: TSX               ; ba 
a092: INX               ; e8 
a093: LDA  $0100,X      ; bd 00 01
a096: JSR  $a5c9        ; 20 c9 a5
a099: LDA  #$20         ; a9 20
a09b: JSR  $a5a5        ; 20 a5 a5
a09e: INX               ; e8 
a09f: BNE  $a093        ; d0 f2
a0a1: CLI               ; 58 
a0a2: RTS               ; 60 
a0a3: .byte $22   ;?
a0a4: .byte $57   ;?
a0a5: .byte $0b   ;?
a0a6: AND  ($40)        ; 32 40
a0a8: .byte $22   ;?
a0a9: .byte $e2   ;?
a0aa: PHP               ; 08 
a0ab: ORA  $1a01,X      ; 0d 01 1a
a0ae: SBC  ($06)        ; f2 06
a0b0: ASL  $1501        ; 0e 01 15
a0b3: SBC  ($06)        ; f2 06
a0b5: ASL  $1080        ; 0e 80 10
a0b8: LDA  $0e04,X      ; ad 04 0e
a0bb: BNE  $a0c8        ; d0 0b
a0bd: LDA  #$01         ; a9 01
a0bf: STA  $0d08,X      ; 8d 08 0d
a0c2: CMP  ($01)        ; d2 01
a0c4: ASL  $0e,X        ; 06 0e
a0c6: AND  ($40)        ; 32 40
a0c8: SBC  ($54)        ; f2 54
a0ca: TSB  $80          ; 04 80
a0cc: .byte $03   ;?
a0cd: JMP  ($a178)      ; 4c 78 a1
a0d0: LDA  $0457,X      ; ad 57 04
a0d3: .byte $c2   ;?
a0d4: RTI               ; 40 
a0d5: TRB  $04          ; 54 04
a0d7: CLI               ; 58 
a0d8: LDA  $a9          ; a5 a9
a0da: BEQ  $a0de        ; f0 02
a0dc: DEC  $a9,X        ; c6 a9
a0de: LDA  $a8          ; a5 a8
a0e0: BEQ  $a0e4        ; f0 02
a0e2: DEC  $a8,X        ; c6 a8
a0e4: .byte $0f   ;?
a0e5: .byte $4f   ;?
a0e6: DEA               ; 3a 
a0e7: JSR  $1459        ; 20 59 14
a0ea: LDA  $d8          ; a5 d8
a0ec: BEQ  $a0f9        ; f0 0b
a0ee: CMP  #$44         ; c9 44
a0f0: BEQ  $a0f9        ; f0 07
a0f2: LDA  $00          ; a5 00
a0f4: AND  #$44         ; 29 44
a0f6: PHA               ; 48 
a0f7: BRA  $a11e        ; 80 25
a0f9: LDA  $00          ; a5 00
a0fb: AND  #$44         ; 29 44
a0fd: PHA               ; 48 
a0fe: EOR  $d8          ; 45 d8
a100: BEQ  $a11e        ; f0 1c
a102: CMP  #$44         ; c9 44
a104: BEQ  $a11e        ; f0 18
a106: CMP  #$40         ; c9 40
a108: BEQ  $a114        ; f0 0a
a10a: LDA  $12d2,X      ; ad d2 12
a10d: BEQ  $a11e        ; f0 0f
a10f: INC  $12d2        ; ee d2 12
a112: BRA  $a11e        ; 80 0a
a114: LDA  $12d2,X      ; ad d2 12
a117: CMP  #$b1         ; c9 b1
a119: BEQ  $a11e        ; f0 03
a11b: DEC  $12d2        ; ce d2 12
a11e: PLA               ; 68 
a11f: STA  $d8          ; 85 d8
a121: .byte $3f   ;?
a122: .byte $93   ;?
a123: .byte $07   ;?
a124: JSR  $123e        ; 20 3e 12
a127: CMP  ($40)        ; d2 40
a129: TRB  $04          ; 54 04
a12b: AND  ($40)        ; 32 40
a12d: .byte $22   ;?
a12e: .byte $37   ;?
a12f: .byte $0b   ;?
a130: .byte $0f   ;?
a131: BIT  ($cb35)      ; 3c 35 cb
a134: LDA  $40          ; a5 40
a136: LDX  $41,X        ; a6 41
a138: LDY  $42          ; a4 42
a13a: .byte $22   ;?
a13b: LDA  $43          ; a5 43
a13d: LDX  $44,X        ; a6 44
a13f: LDY  $45          ; a4 45
a141: .byte $22   ;?
a142: LDA  $46          ; a5 46
a144: LDX  $47,X        ; a6 47
a146: .byte $22   ;?
a147: .byte $23   ;?
a148: LDA  $38          ; a5 38
a14a: .byte $7f   ;?
a14b: .byte $93   ;?
a14c: ASL  $20,X        ; 06 20
a14e: .byte $53   ;?
a14f: TRB  $20          ; 14 20
a151: SEC               ; 38 
a152: ORA  ($33)        ; 12 33
a154: AND  ($85)        ; 32 85
a156: LSR  $86,X        ; 46 86
a158: .byte $47   ;?
a159: AND  ($85)        ; 32 85
a15b: .byte $43   ;?
a15c: STX  $44,X        ; 86 44
a15e: STY  $45          ; 84 45
a160: AND  ($85)        ; 32 85
a162: RTI               ; 40 
a163: STX  $41,X        ; 86 41
a165: STY  $42          ; 84 42
a167: .byte $db   ;?
a168: AND  ($40)        ; 32 40
a16a: JSR  $13c3        ; 20 c3 13
a16d: .byte $1f   ;?
a16e: .byte $39   ;?
a16f: .byte $07   ;?
a170: .byte $5f   ;?
a171: BIT  ($2204)      ; 3c 04 22
a174: .byte $17   ;?
a175: .byte $39   ;?
a176: AND  ($40)        ; 32 40
a178: JSR  $be3b        ; 20 3b be
a17b: JSR  $bf1e        ; 20 1e bf
a17e: LDA  $137d,X      ; ad 7d 13
a181: JSR  $a186        ; 20 86 a1
a184: AND  ($40)        ; 32 40
a186: ASL               ; 0a 
a187: TAX               ; aa 
a188: JMP  ($a18b,X)    ; 7c 8b a1
a18b: .byte $99   ;?
a18c: LDA  ($99,X)      ; a1 99
a18e: LDA  ($9d,X)      ; a1 9d
a190: LDA  ($9d,X)      ; a1 9d
a192: LDA  ($9d,X)      ; a1 9d
a194: LDA  ($9d,X)      ; a1 9d
a196: LDA  ($9d,X)      ; a1 9d
a198: LDA  ($ad,X)      ; a1 ad
a19a: .byte $02   ;?
a19b: ORA  $a560,X      ; 0d 60 a5
a19e: RTI               ; 40 
a19f: LDX  $41,X        ; a6 41
a1a1: LDY  $42          ; a4 42
a1a3: .byte $22   ;?
a1a4: LDA  $43          ; a5 43
a1a6: LDX  $44,X        ; a6 44
a1a8: LDY  $45          ; a4 45
a1aa: .byte $22   ;?
a1ab: LDA  $46          ; a5 46
a1ad: LDX  $47,X        ; a6 47
a1af: .byte $22   ;?
a1b0: JSR  $bf4d        ; 20 4d bf
a1b3: JSR  $c110        ; 20 10 c1
a1b6: AND  ($85)        ; 32 85
a1b8: LSR  $86,X        ; 46 86
a1ba: .byte $47   ;?
a1bb: AND  ($85)        ; 32 85
a1bd: .byte $43   ;?
a1be: STX  $44,X        ; 86 44
a1c0: STY  $45          ; 84 45
a1c2: AND  ($85)        ; 32 85
a1c4: RTI               ; 40 
a1c5: STX  $41,X        ; 86 41
a1c7: STY  $42          ; 84 42
a1c9: JSR  $be97        ; 20 97 be
a1cc: JSR  $bed8        ; 20 d8 be
a1cf: JMP  ($bec6)      ; 4c c6 be
a1d2: LDA  #$03         ; a9 03
a1d4: STA  $123f,X      ; 8d 3f 12
a1d7: LDA  #$a2         ; a9 a2
a1d9: STA  $1240,X      ; 8d 40 12
a1dc: .byte $8f   ;?
a1dd: STY  $10,X        ; 94 10
a1df: LDA  $ab          ; a5 ab
a1e1: BNE  $a1ed        ; d0 0a
a1e3: LDA  $ac          ; a5 ac
a1e5: BNE  $a1eb        ; d0 04
a1e7: .byte $87   ;?
a1e8: STY  $80,X        ; 94 80
a1ea: TSB  $c6          ; 04 c6
a1ec: LDY  ($abc6)      ; ac c6 ab
a1ef: .byte $af   ;?
a1f0: STA  $10          ; 95 10
a1f2: LDA  $b5          ; a5 b5
a1f4: BNE  $a200        ; d0 0a
a1f6: LDA  $b6          ; a5 b6
a1f8: BNE  $a1fe        ; d0 04
a1fa: .byte $a7   ;?
a1fb: STA  $80          ; 95 80
a1fd: TSB  $c6          ; 04 c6
a1ff: LDX  $c6,Y        ; b6 c6
a201: LDA  $60          ; b5 60
a203: LDA  #$34         ; a9 34
a205: STA  $123f,X      ; 8d 3f 12
a208: LDA  #$a2         ; a9 a2
a20a: STA  $1240,X      ; 8d 40 12
a20d: .byte $9f   ;?
a20e: STY  $10,X        ; 94 10
a210: LDA  $ad          ; a5 ad
a212: BNE  $a21e        ; d0 0a
a214: LDA  $ae          ; a5 ae
a216: BNE  $a21c        ; d0 04
a218: .byte $97   ;?
a219: STY  $80,X        ; 94 80
a21b: TSB  $c6          ; 04 c6
a21d: LDX  $adc6        ; ae c6 ad
a220: .byte $bf   ;?
a221: STA  $10          ; 95 10
a223: LDA  $b7          ; a5 b7
a225: BNE  $a231        ; d0 0a
a227: LDA  $b8          ; a5 b8
a229: BNE  $a22f        ; d0 04
a22b: .byte $b7   ;?
a22c: STA  $80          ; 95 80
a22e: TSB  $c6          ; 04 c6
a230: CLV               ; b8 
a231: DEC  $b7,X        ; c6 b7
a233: RTS               ; 60 
a234: LDA  #$65         ; a9 65
a236: STA  $123f,X      ; 8d 3f 12
a239: LDA  #$a2         ; a9 a2
a23b: STA  $1240,X      ; 8d 40 12
a23e: .byte $af   ;?
a23f: STY  $10,X        ; 94 10
a241: LDA  $af          ; a5 af
a243: BNE  $a24f        ; d0 0a
a245: LDA  $b0          ; a5 b0
a247: BNE  $a24d        ; d0 04
a249: .byte $a7   ;?
a24a: STY  $80,X        ; 94 80
a24c: TSB  $c6          ; 04 c6
a24e: BCS  $a216        ; b0 c6
a250: .byte $af   ;?
a251: .byte $cf   ;?
a252: STA  $10          ; 95 10
a254: LDA  $b9          ; a5 b9
a256: BNE  $a262        ; d0 0a
a258: LDA  $ba          ; a5 ba
a25a: BNE  $a260        ; d0 04
a25c: .byte $c7   ;?
a25d: STA  $80          ; 95 80
a25f: TSB  $c6          ; 04 c6
a261: TSX               ; ba 
a262: DEC  $b9,X        ; c6 b9
a264: RTS               ; 60 
a265: LDA  #$96         ; a9 96
a267: STA  $123f,X      ; 8d 3f 12
a26a: LDA  #$a2         ; a9 a2
a26c: STA  $1240,X      ; 8d 40 12
a26f: .byte $bf   ;?
a270: STY  $10,X        ; 94 10
a272: LDA  $b1          ; a5 b1
a274: BNE  $a280        ; d0 0a
a276: LDA  $b2          ; a5 b2
a278: BNE  $a27e        ; d0 04
a27a: .byte $b7   ;?
a27b: STY  $80,X        ; 94 80
a27d: TSB  $c6          ; 04 c6
a27f: LDA  ($c6)        ; b2 c6
a281: LDA  ($df),Y      ; b1 df
a283: STA  $10          ; 95 10
a285: LDA  $bb          ; a5 bb
a287: BNE  $a293        ; d0 0a
a289: LDA  $bc          ; a5 bc
a28b: BNE  $a291        ; d0 04
a28d: .byte $d7   ;?
a28e: STA  $80          ; 95 80
a290: TSB  $c6          ; 04 c6
a292: LDY  ($bbc6)      ; bc c6 bb
a295: RTS               ; 60 
a296: LDA  #$c7         ; a9 c7
a298: STA  $123f,X      ; 8d 3f 12
a29b: LDA  #$a2         ; a9 a2
a29d: STA  $1240,X      ; 8d 40 12
a2a0: .byte $cf   ;?
a2a1: STY  $10,X        ; 94 10
a2a3: LDA  $b3          ; a5 b3
a2a5: BNE  $a2b1        ; d0 0a
a2a7: LDA  $b4          ; a5 b4
a2a9: BNE  $a2af        ; d0 04
a2ab: .byte $c7   ;?
a2ac: STY  $80,X        ; 94 80
a2ae: TSB  $c6          ; 04 c6
a2b0: LDY  $c6,X        ; b4 c6
a2b2: .byte $b3   ;?
a2b3: .byte $ef   ;?
a2b4: STA  $10          ; 95 10
a2b6: LDA  $bd          ; a5 bd
a2b8: BNE  $a2c4        ; d0 0a
a2ba: LDA  $be          ; a5 be
a2bc: BNE  $a2c2        ; d0 04
a2be: .byte $e7   ;?
a2bf: STA  $80          ; 95 80
a2c1: TSB  $c6          ; 04 c6
a2c3: LDX  $bdc6        ; be c6 bd
a2c6: RTS               ; 60 
a2c7: LDA  #$e5         ; a9 e5
a2c9: STA  $123f,X      ; 8d 3f 12
a2cc: LDA  #$a2         ; a9 a2
a2ce: STA  $1240,X      ; 8d 40 12
a2d1: .byte $ff   ;?
a2d2: STA  $10          ; 95 10
a2d4: LDA  $bf          ; a5 bf
a2d6: BNE  $a2e2        ; d0 0a
a2d8: LDA  $c0          ; a5 c0
a2da: BNE  $a2e0        ; d0 04
a2dc: .byte $f7   ;?
a2dd: STA  $80          ; 95 80
a2df: TSB  $c6          ; 04 c6
a2e1: CPY  #$c6         ; c0 c6
a2e3: .byte $bf   ;?
a2e4: RTS               ; 60 
a2e5: LDA  #$03         ; a9 03
a2e7: STA  $123f,X      ; 8d 3f 12
a2ea: LDA  #$a3         ; a9 a3
a2ec: STA  $1240,X      ; 8d 40 12
a2ef: .byte $8f   ;?
a2f0: STX  $10,Y        ; 96 10
a2f2: LDA  $c1          ; a5 c1
a2f4: BNE  $a300        ; d0 0a
a2f6: LDA  $c2          ; a5 c2
a2f8: BNE  $a2fe        ; d0 04
a2fa: .byte $87   ;?
a2fb: STX  $80,Y        ; 96 80
a2fd: TSB  $c6          ; 04 c6
a2ff: .byte $c2   ;?
a300: DEC  $c1,X        ; c6 c1
a302: RTS               ; 60 
a303: LDA  #$21         ; a9 21
a305: STA  $123f,X      ; 8d 3f 12
a308: LDA  #$a3         ; a9 a3
a30a: STA  $1240,X      ; 8d 40 12
a30d: .byte $9f   ;?
a30e: STX  $10,Y        ; 96 10
a310: LDA  $c3          ; a5 c3
a312: BNE  $a31e        ; d0 0a
a314: LDA  $c4          ; a5 c4
a316: BNE  $a31c        ; d0 04
a318: .byte $97   ;?
a319: STX  $80,Y        ; 96 80
a31b: TSB  $c6          ; 04 c6
a31d: CPY  $c6          ; c4 c6
a31f: .byte $c3   ;?
a320: RTS               ; 60 
a321: LDA  #$3f         ; a9 3f
a323: STA  $123f,X      ; 8d 3f 12
a326: LDA  #$a3         ; a9 a3
a328: STA  $1240,X      ; 8d 40 12
a32b: .byte $af   ;?
a32c: STX  $10,Y        ; 96 10
a32e: LDA  $c5          ; a5 c5
a330: BNE  $a33c        ; d0 0a
a332: LDA  $c6          ; a5 c6
a334: BNE  $a33a        ; d0 04
a336: .byte $a7   ;?
a337: STX  $80,Y        ; 96 80
a339: TSB  $c6          ; 04 c6
a33b: DEC  $c6,X        ; c6 c6
a33d: CMP  $60          ; c5 60
a33f: LDA  #$d2         ; a9 d2
a341: STA  $123f,X      ; 8d 3f 12
a344: LDA  #$a1         ; a9 a1
a346: STA  $1240,X      ; 8d 40 12
a349: .byte $1f   ;?
a34a: .byte $99   ;?
a34b: ORA  ($60,X)      ; 01 60
a34d: .byte $97   ;?
a34e: .byte $99   ;?
a34f: CMP  ($40)        ; d2 40
a351: TRB  $04          ; 54 04
a353: CLI               ; 58 
a354: .byte $4f   ;?
a355: .byte $93   ;?
a356: ASL  $20,X        ; 06 20
a358: DEC  $13,X        ; c6 13
a35a: JSR  $1241        ; 20 41 12
a35d: LDA  $aa          ; a5 aa
a35f: BEQ  $a363        ; f0 02
a361: DEC  $aa,X        ; c6 aa
a363: JSR  $e9c3        ; 20 c3 e9
a366: .byte $78   ;?
a367: .byte $17   ;?
a368: .byte $99   ;?
a369: RTS               ; 60 
a36a: LDA  $1009,X      ; ad 09 10
a36d: BEQ  $a377        ; f0 08
a36f: LDA  $d1          ; a5 d1
a371: CMP  #$ff         ; c9 ff
a373: BEQ  $a377        ; f0 02
a375: INC  $d1,X        ; e6 d1
a377: LDA  #$84         ; a9 84
a379: STA  $1242,X      ; 8d 42 12
a37c: LDA  #$a3         ; a9 a3
a37e: STA  $1243,X      ; 8d 43 12
a381: .byte $97   ;?
a382: STA  $a960,X      ; 9d 60 a9
a385: LDX  #$8d         ; a2 8d
a387: .byte $42   ;?
a388: ORA  ($a9)        ; 12 a9
a38a: .byte $a3   ;?
a38b: STA  $1243,X      ; 8d 43 12
a38e: .byte $df   ;?
a38f: STY  $10,X        ; 94 10
a391: LDA  $c7          ; a5 c7
a393: BNE  $a39f        ; d0 0a
a395: LDA  $c8          ; a5 c8
a397: BNE  $a39d        ; d0 04
a399: .byte $d7   ;?
a39a: STY  $80,X        ; 94 80
a39c: TSB  $c6          ; 04 c6
a39e: INY               ; c8 
a39f: DEC  $c7,X        ; c6 c7
a3a1: RTS               ; 60 
a3a2: LDA  #$c2         ; a9 c2
a3a4: STA  $1242,X      ; 8d 42 12
a3a7: LDA  #$a3         ; a9 a3
a3a9: STA  $1243,X      ; 8d 43 12
a3ac: .byte $ef   ;?
a3ad: STY  $10,X        ; 94 10
a3af: LDA  $c9          ; a5 c9
a3b1: BNE  $a3bd        ; d0 0a
a3b3: LDA  $ca          ; a5 ca
a3b5: BNE  $a3bb        ; d0 04
a3b7: .byte $e7   ;?
a3b8: STY  $80,X        ; 94 80
a3ba: TSB  $c6          ; 04 c6
a3bc: DEX               ; ca 
a3bd: DEC  $c9,X        ; c6 c9
a3bf: .byte $97   ;?
a3c0: STA  $a960,X      ; 9d 60 a9
a3c3: CPX  #$8d         ; e0 8d
a3c5: .byte $42   ;?
a3c6: ORA  ($a9)        ; 12 a9
a3c8: .byte $a3   ;?
a3c9: STA  $1243,X      ; 8d 43 12
a3cc: .byte $ff   ;?
a3cd: STY  $10,X        ; 94 10
a3cf: LDA  $cb          ; a5 cb
a3d1: BNE  $a3dd        ; d0 0a
a3d3: LDA  $cc          ; a5 cc
a3d5: BNE  $a3db        ; d0 04
a3d7: .byte $f7   ;?
a3d8: STY  $80,X        ; 94 80
a3da: TSB  $c6          ; 04 c6
a3dc: CPY  ($cbc6)      ; cc c6 cb
a3df: RTS               ; 60 
a3e0: LDA  #$ed         ; a9 ed
a3e2: STA  $1242,X      ; 8d 42 12
a3e5: LDA  #$a3         ; a9 a3
a3e7: STA  $1243,X      ; 8d 43 12
a3ea: .byte $97   ;?
a3eb: STA  $a960,X      ; 9d 60 a9
a3ee: SED               ; f8 
a3ef: STA  $1242,X      ; 8d 42 12
a3f2: LDA  #$a3         ; a9 a3
a3f4: STA  $1243,X      ; 8d 43 12
a3f7: RTS               ; 60 
a3f8: LDA  #$05         ; a9 05
a3fa: STA  $1242,X      ; 8d 42 12
a3fd: LDA  #$a4         ; a9 a4
a3ff: STA  $1243,X      ; 8d 43 12
a402: .byte $97   ;?
a403: STA  $a960,X      ; 9d 60 a9
a406: .byte $19   ;?
a407: STA  $1242,X      ; 8d 42 12
a40a: LDA  #$a4         ; a9 a4
a40c: STA  $1243,X      ; 8d 43 12
a40f: .byte $5f   ;?
a410: .byte $93   ;?
a411: ASL  $20,X        ; 06 20
a413: CMP  #$13         ; c9 13
a415: JSR  $1244        ; 20 44 12
a418: RTS               ; 60 
a419: LDA  #$26         ; a9 26
a41b: STA  $1242,X      ; 8d 42 12
a41e: LDA  #$a4         ; a9 a4
a420: STA  $1243,X      ; 8d 43 12
a423: .byte $97   ;?
a424: STA  $a960,X      ; 9d 60 a9
a427: ROR               ; 6a 
a428: STA  $1242,X      ; 8d 42 12
a42b: LDA  #$a3         ; a9 a3
a42d: STA  $1243,X      ; 8d 43 12
a430: RTS               ; 60 
a431: LDA  #$4f         ; a9 4f
a433: STA  $1245,X      ; 8d 45 12
a436: LDA  #$a4         ; a9 a4
a438: STA  $1246,X      ; 8d 46 12
a43b: .byte $8f   ;?
a43c: STA  $10          ; 95 10
a43e: LDA  $cd          ; a5 cd
a440: BNE  $a44c        ; d0 0a
a442: LDA  $ce          ; a5 ce
a444: BNE  $a44a        ; d0 04
a446: .byte $87   ;?
a447: STA  $80          ; 95 80
a449: TSB  $c6          ; 04 c6
a44b: DEC  $cdc6        ; ce c6 cd
a44e: RTS               ; 60 
a44f: LDA  #$6d         ; a9 6d
a451: STA  $1245,X      ; 8d 45 12
a454: LDA  #$a4         ; a9 a4
a456: STA  $1246,X      ; 8d 46 12
a459: .byte $9f   ;?
a45a: STA  $10          ; 95 10
a45c: LDA  $cf          ; a5 cf
a45e: BNE  $a46a        ; d0 0a
a460: LDA  $d0          ; a5 d0
a462: BNE  $a468        ; d0 04
a464: .byte $97   ;?
a465: STA  $80          ; 95 80
a467: TSB  $c6          ; 04 c6
a469: BNE  $a431        ; d0 c6
a46b: .byte $cf   ;?
a46c: RTS               ; 60 
a46d: LDA  #$78         ; a9 78
a46f: STA  $1245,X      ; 8d 45 12
a472: LDA  #$a4         ; a9 a4
a474: STA  $1246,X      ; 8d 46 12
a477: RTS               ; 60 
a478: LDA  #$83         ; a9 83
a47a: STA  $1245,X      ; 8d 45 12
a47d: LDA  #$a4         ; a9 a4
a47f: STA  $1246,X      ; 8d 46 12
a482: RTS               ; 60 
a483: LDA  #$8e         ; a9 8e
a485: STA  $1245,X      ; 8d 45 12
a488: LDA  #$a4         ; a9 a4
a48a: STA  $1246,X      ; 8d 46 12
a48d: RTS               ; 60 
a48e: LDA  #$99         ; a9 99
a490: STA  $1245,X      ; 8d 45 12
a493: LDA  #$a4         ; a9 a4
a495: STA  $1246,X      ; 8d 46 12
a498: RTS               ; 60 
a499: LDA  #$a4         ; a9 a4
a49b: STA  $1245,X      ; 8d 45 12
a49e: LDA  #$a4         ; a9 a4
a4a0: STA  $1246,X      ; 8d 46 12
a4a3: RTS               ; 60 
a4a4: LDA  #$af         ; a9 af
a4a6: STA  $1245,X      ; 8d 45 12
a4a9: LDA  #$a4         ; a9 a4
a4ab: STA  $1246,X      ; 8d 46 12
a4ae: RTS               ; 60 
a4af: LDA  #$ba         ; a9 ba
a4b1: STA  $1245,X      ; 8d 45 12
a4b4: LDA  #$a4         ; a9 a4
a4b6: STA  $1246,X      ; 8d 46 12
a4b9: RTS               ; 60 
a4ba: LDA  #$31         ; a9 31
a4bc: STA  $1245,X      ; 8d 45 12
a4bf: LDA  #$a4         ; a9 a4
a4c1: STA  $1246,X      ; 8d 46 12
a4c4: RTS               ; 60 
a4c5: CPX  #$00         ; e0 00
a4c7: BEQ  $a4cf        ; f0 06
a4c9: JSR  $a4d0        ; 20 d0 a4
a4cc: DEX               ; ca 
a4cd: BNE  $a4c9        ; d0 fa
a4cf: RTS               ; 60 
a4d0: PHX               ; da 
a4d1: LDX  #$64         ; a2 64
a4d3: JSR  $a4f7        ; 20 f7 a4
a4d6: PLX               ; fa 
a4d7: RTS               ; 60 
a4d8: CPX  #$00         ; e0 00
a4da: BEQ  $a4e2        ; f0 06
a4dc: JSR  $a4e3        ; 20 e3 a4
a4df: DEX               ; ca 
a4e0: BNE  $a4dc        ; d0 fa
a4e2: RTS               ; 60 
a4e3: PHX               ; da 
a4e4: LDX  #$0a         ; a2 0a
a4e6: JSR  $a4f7        ; 20 f7 a4
a4e9: PLX               ; fa 
a4ea: RTS               ; 60 
a4eb: CPX  #$00         ; e0 00
a4ed: BEQ  $a4f6        ; f0 07
a4ef: CPX  #$01         ; e0 01
a4f1: BEQ  $a512        ; f0 1f
a4f3: JSR  $a4f7        ; 20 f7 a4
a4f6: RTS               ; 60 
a4f7: .byte $22   ;?
a4f8: PHP               ; 08 
a4f9: TSX               ; ba 
a4fa: LDA  $0101,X      ; bd 01 01
a4fd: PLP               ; 28 
a4fe: AND  #$04         ; 29 04
a500: BNE  $a507        ; d0 05
a502: SBC  ($54)        ; f2 54
a504: TSB  $40          ; 04 40
a506: .byte $03   ;?
a507: AND  ($80)        ; 32 80
a509: ASL               ; 0a 
a50a: AND  ($86)        ; 32 86
a50c: LDA  #$a6         ; a9 a6
a50e: LDA  #$d0         ; a9 d0
a510: CPX  ($a260)      ; fc 60 a2
a513: ORA  ($5a,X)      ; 01 5a
a515: LDY  #$0a         ; a0 0a
a517: JSR  $a51f        ; 20 1f a5
a51a: DEY               ; 88 
a51b: BNE  $a517        ; d0 fa
a51d: PLY               ; 7a 
a51e: RTS               ; 60 
a51f: PHX               ; da 
a520: PHY               ; 5a 
a521: LDY  #$9b         ; a0 9b
a523: .byte $3f   ;?
a524: ORA  #$02         ; 09 02
a526: LDY  #$4d         ; a0 4d
a528: JSR  $a533        ; 20 33 a5
a52b: BNE  $a528        ; d0 fb
a52d: DEX               ; ca 
a52e: BNE  $a521        ; d0 f1
a530: PLY               ; 7a 
a531: PLX               ; fa 
a532: RTS               ; 60 
a533: .byte $cb   ;?
a534: .byte $db   ;?
a535: NOP               ; ea 
a536: NOP               ; ea 
a537: NOP               ; ea 
a538: DEY               ; 88 
a539: RTS               ; 60 
a53a: PHX               ; da 
a53b: PHY               ; 5a 
a53c: LDY  #$14         ; a0 14
a53e: JSR  $a549        ; 20 49 a5
a541: BNE  $a53e        ; d0 fb
a543: DEX               ; ca 
a544: BNE  $a53c        ; d0 f6
a546: PLY               ; 7a 
a547: PLX               ; fa 
a548: RTS               ; 60 
a549: PHA               ; 48 
a54a: NOP               ; ea 
a54b: PLA               ; 68 
a54c: DEY               ; 88 
a54d: RTS               ; 60 
a54e: RTS               ; 60 
a54f: .byte $22   ;?
a550: .byte $77   ;?
a551: .byte $0b   ;?
a552: STA  $3c          ; 85 3c
a554: LDA  $0452,X      ; ad 52 04
a557: AND  ($40)        ; 32 40
a559: .byte $22   ;?
a55a: .byte $67   ;?
a55b: .byte $0b   ;?
a55c: .byte $57   ;?
a55d: .byte $0b   ;?
a55e: LDA  $044d,X      ; ad 4d 04
a561: ORA  #$01         ; 09 01
a563: STA  $08          ; 85 08
a565: AND  ($40)        ; 32 40
a567: LDA  $ebba,X      ; bd ba eb
a56a: STA  $0459,X      ; 8d 59 04
a56d: LDA  $ebbb,X      ; bd bb eb
a570: STA  $045a,X      ; 8d 5a 04
a573: LDA  $ebba,X      ; bd ba eb
a576: STA  $045d,X      ; 8d 5d 04
a579: LDA  $ebbb,X      ; bd bb eb
a57c: STA  $045e,X      ; 8d 5e 04
a57f: LDA  ($0f)        ; b2 0f
a581: ROL  $0fb2        ; 3e b2 0f
a584: .byte $3f   ;?
a585: LDA  #$00         ; a9 00
a587: STA  $0458,X      ; 8d 58 04
a58a: STA  $045c,X      ; 8d 5c 04
a58d: RTS               ; 60 
a58e: .byte $87   ;?
a58f: .byte $39   ;?
a590: RTS               ; 60 
a591: .byte $07   ;?
a592: .byte $39   ;?
a593: RTS               ; 60 
a594: SEC               ; 38 
a595: LDA  ($00)        ; b2 00
a597: EOR  ($0f,X)      ; 41 0f
a599: BIT  ($2005)      ; 3c 05 20
a59c: LDA  ($a5,X)      ; a1 a5
a59e: STA  $41          ; 85 41
a5a0: RTS               ; 60 
a5a1: LDA  $38          ; a5 38
a5a3: CLC               ; 18 
a5a4: RTS               ; 60 
a5a5: .byte $df   ;?
a5a6: .byte $99   ;?
a5a7: ORA  $5f          ; 05 5f
a5a9: BIT  ($85fd)      ; 3c fd 85
a5ac: SEC               ; 38 
a5ad: RTS               ; 60 
a5ae: .byte $df   ;?
a5af: .byte $99   ;?
a5b0: .byte $02   ;?
a5b1: STA  $38          ; 85 38
a5b3: RTS               ; 60 
a5b4: .byte $7f   ;?
a5b5: DEA               ; 3a 
a5b6: ASL  $5f,X        ; 06 5f
a5b8: BIT  ($6ffd)      ; 3c fd 6f
a5bb: BIT  ($60fd)      ; 3c fd 60
a5be: LDA  #$0d         ; a9 0d
a5c0: BRA  $a5a5        ; 80 e3
a5c2: JSR  $a5be        ; 20 be a5
a5c5: LDA  #$0a         ; a9 0a
a5c7: BRA  $a5a5        ; 80 dc
a5c9: PHA               ; 48 
a5ca: LSR               ; 4a 
a5cb: LSR               ; 4a 
a5cc: LSR               ; 4a 
a5cd: LSR               ; 4a 
a5ce: JSR  $a5d4        ; 20 d4 a5
a5d1: PLA               ; 68 
a5d2: AND  #$0f         ; 29 0f
a5d4: JSR  $a5da        ; 20 da a5
a5d7: JMP  ($a5a5)      ; 4c a5 a5
a5da: CMP  #$0a         ; c9 0a
a5dc: BCC  $a5e0        ; 90 02
a5de: ADC  #$06         ; 69 06
a5e0: ADC  #$30         ; 69 30
a5e2: RTS               ; 60 
a5e3: CMP  #$61         ; c9 61
a5e5: BCC  $a5eb        ; 90 04
a5e7: SBC  #$28         ; e9 28
a5e9: BRA  $a5f1        ; 80 06
a5eb: CMP  #$41         ; c9 41
a5ed: BCC  $a5f1        ; 90 02
a5ef: SBC  #$08         ; e9 08
a5f1: SBC  #$2f         ; e9 2f
a5f3: RTS               ; 60 
a5f4: LDA  ($40,X)      ; a1 40
a5f6: JSR  $a5e3        ; 20 e3 a5
a5f9: ASL               ; 0a 
a5fa: ASL               ; 0a 
a5fb: ASL               ; 0a 
a5fc: ASL               ; 0a 
a5fd: STA  $42          ; 85 42
a5ff: JSR  $a84a        ; 20 4a a8
a602: LDA  ($40,X)      ; a1 40
a604: JSR  $a5e3        ; 20 e3 a5
a607: JSR  $a84a        ; 20 4a a8
a60a: ORA  $42          ; 05 42
a60c: RTS               ; 60 
a60d: LDA  ($c0)        ; b2 c0
a60f: BRK               ; 00 
a610: LDA  ($b0)        ; b2 b0
a612: TSB  $b2          ; 04 b2
a614: BPL  $a617        ; 10 01
a616: LDA  ($7f)        ; b2 7f
a618: ORA  $77          ; 05 77
a61a: AND  ($b2)        ; 32 b2
a61c: BEQ  $a624        ; f0 06
a61e: LDA  ($00)        ; b2 00
a620: .byte $02   ;?
a621: LDA  ($00)        ; b2 00
a623: .byte $03   ;?
a624: LDA  #$00         ; a9 00
a626: STA  $0442,X      ; 8d 42 04
a629: LDA  #$1e         ; a9 1e
a62b: STA  $046b,X      ; 8d 6b 04
a62e: LDA  ($0c)        ; b2 0c
a630: .byte $07   ;?
a631: LDA  #$f5         ; a9 f5
a633: STA  $047c,X      ; 8d 7c 04
a636: LDA  ($00)        ; b2 00
a638: PHP               ; 08 
a639: RTS               ; 60 
a63a: LDA  ($48)        ; b2 48
a63c: RTI               ; 40 
a63d: LDA  ($00)        ; b2 00
a63f: EOR  ($b2,X)      ; 41 b2
a641: .byte $ff   ;?
a642: .byte $42   ;?
a643: LDA  ($00)        ; b2 00
a645: .byte $43   ;?
a646: LDA  ($01)        ; b2 01
a648: TSB  $b2          ; 44 b2
a64a: EOR  ($46),Y      ; 51 46
a64c: LDA  ($a6)        ; b2 a6
a64e: .byte $47   ;?
a64f: BRA  $a67d        ; 80 2c
a651: LDA  ($00)        ; b2 00
a653: RTI               ; 40 
a654: LDA  ($02)        ; b2 02
a656: EOR  ($b2,X)      ; 41 b2
a658: .byte $3f   ;?
a659: .byte $42   ;?
a65a: LDA  ($04)        ; b2 04
a65c: .byte $43   ;?
a65d: LDA  ($01)        ; b2 01
a65f: TSB  $b2          ; 44 b2
a661: PLA               ; 68 
a662: LSR  $b2,X        ; 46 b2
a664: LDX  $47,X        ; a6 47
a666: BRA  $a67d        ; 80 15
a668: LDA  ($00)        ; b2 00
a66a: RTI               ; 40 
a66b: LDA  ($10)        ; b2 10
a66d: EOR  ($b2,X)      ; 41 b2
a66f: .byte $ff   ;?
a670: .byte $42   ;?
a671: LDA  ($31)        ; b2 31
a673: .byte $43   ;?
a674: LDA  ($a7)        ; b2 a7
a676: LSR  $b2,X        ; 46 b2
a678: LDX  $47,X        ; a6 47
a67a: LDA  ($01)        ; b2 01
a67c: TSB  $a6          ; 44 a6
a67e: TSB  $bd          ; 44 bd
a680: CPY  $eb          ; c4 eb
a682: STA  ($40,X)      ; 81 40
a684: LDA  ($40,X)      ; a1 40
a686: CMP  $ebc4,X      ; dd c4 eb
a689: BEQ  $a68d        ; f0 02
a68b: BRA  $a6a5        ; 80 18
a68d: DEX               ; ca 
a68e: BPL  $a67f        ; 10 ef
a690: INC  $40,X        ; e6 40
a692: BNE  $a696        ; d0 02
a694: INC  $41,X        ; e6 41
a696: LDA  $41          ; a5 41
a698: CMP  $43          ; c5 43
a69a: BNE  $a6a0        ; d0 04
a69c: LDA  $40          ; a5 40
a69e: CMP  $42          ; c5 42
a6a0: BCC  $a67d        ; 90 db
a6a2: JMP  ($0046)      ; 6c 46 00
a6a5: .byte $f7   ;?
a6a6: EOR  $a260,X      ; 4d 60 a2
a6a9: ASL               ; 0a 
a6aa: LDA  ($38)        ; b2 38
a6ac: RTI               ; 40 
a6ad: LDA  ($12)        ; b2 12
a6af: EOR  ($a9,X)      ; 41 a9
a6b1: JMP  ($4081)      ; 4c 81 40
a6b4: PHX               ; da 
a6b5: LDX  #$01         ; a2 01
a6b7: LDA  #$6a         ; a9 6a
a6b9: STA  ($40),Y      ; 91 40
a6bb: INX               ; e8 
a6bc: LDA  #$a0         ; a9 a0
a6be: STA  ($40),Y      ; 91 40
a6c0: PLX               ; fa 
a6c1: CLC               ; 18 
a6c2: LDA  $40          ; a5 40
a6c4: ADC  #$03         ; 69 03
a6c6: STA  $40          ; 85 40
a6c8: LDA  $41          ; a5 41
a6ca: ADC  #$00         ; 69 00
a6cc: STA  $41          ; 85 41
a6ce: DEX               ; ca 
a6cf: BNE  $a6b0        ; d0 df
a6d1: LDA  #$d2         ; a9 d2
a6d3: STA  $123f,X      ; 8d 3f 12
a6d6: LDA  #$a1         ; a9 a1
a6d8: STA  $1240,X      ; 8d 40 12
a6db: LDA  #$6a         ; a9 6a
a6dd: STA  $1242,X      ; 8d 42 12
a6e0: LDA  #$a3         ; a9 a3
a6e2: STA  $1243,X      ; 8d 43 12
a6e5: LDA  #$31         ; a9 31
a6e7: STA  $1245,X      ; 8d 45 12
a6ea: LDA  #$a4         ; a9 a4
a6ec: STA  $1246,X      ; 8d 46 12
a6ef: .byte $77   ;?
a6f0: .byte $93   ;?
a6f1: LDA  #$be         ; a9 be
a6f3: STA  $1239,X      ; 8d 39 12
a6f6: LDA  #$b3         ; a9 b3
a6f8: STA  $123a,X      ; 8d 3a 12
a6fb: .byte $f7   ;?
a6fc: .byte $93   ;?
a6fd: LDA  #$58         ; a9 58
a6ff: STA  $123c,X      ; 8d 3c 12
a702: LDA  #$b4         ; a9 b4
a704: STA  $123d,X      ; 8d 3d 12
a707: LDA  #$ba         ; a9 ba
a709: STA  $1254,X      ; 8d 54 12
a70c: LDA  #$e9         ; a9 e9
a70e: STA  $1255,X      ; 8d 55 12
a711: .byte $67   ;?
a712: .byte $93   ;?
a713: LDA  #$a2         ; a9 a2
a715: STA  $1248,X      ; 8d 48 12
a718: LDA  #$e4         ; a9 e4
a71a: STA  $1249,X      ; 8d 49 12
a71d: .byte $e7   ;?
a71e: .byte $93   ;?
a71f: JSR  $e8a8        ; 20 a8 e8
a722: JSR  $b447        ; 20 47 b4
a725: LDA  #$95         ; a9 95
a727: STA  $124b,X      ; 8d 4b 12
a72a: LDA  #$bb         ; a9 bb
a72c: STA  $124c,X      ; 8d 4c 12
a72f: LDY  #$01         ; a0 01
a731: BEQ  $a73f        ; f0 0c
a733: DEY               ; 88 
a734: .byte $b9   ;?
a735: DEC  $eb,X        ; c6 eb
a737: .byte $99   ;?
a738: .byte $93   ;?
a739: BRK               ; 00 
a73a: DEY               ; 88 
a73b: CPY  #$ff         ; c0 ff
a73d: BNE  $a734        ; d0 f5
a73f: LDA  #$60         ; a9 60
a741: STA  $12d3,X      ; 8d d3 12
a744: STA  $12d4,X      ; 8d d4 12
a747: STA  $12d5,X      ; 8d d5 12
a74a: LDA  #$ff         ; a9 ff
a74c: STA  $94          ; 85 94
a74e: LDA  #$03         ; a9 03
a750: STA  $95          ; 85 95
a752: LDA  #$08         ; a9 08
a754: STA  $138a,X      ; 8d 8a 13
a757: LDA  #$01         ; a9 01
a759: STA  $127c,X      ; 8d 7c 12
a75c: LDA  #$02         ; a9 02
a75e: STA  $127e,X      ; 8d 7e 12
a761: LDA  #$05         ; a9 05
a763: STA  $1281,X      ; 8d 81 12
a766: LDA  #$07         ; a9 07
a768: STA  $12a4,X      ; 8d a4 12
a76b: LDA  #$08         ; a9 08
a76d: STA  $12c5,X      ; 8d c5 12
a770: LDA  #$00         ; a9 00
a772: STA  $0267,X      ; 8d 67 02
a775: STA  $026b,X      ; 8d 6b 02
a778: LDA  #$80         ; a9 80
a77a: STA  $0268,X      ; 8d 68 02
a77d: STA  $026c,X      ; 8d 6c 02
a780: LDA  #$7f         ; a9 7f
a782: STA  $0269,X      ; 8d 69 02
a785: STA  $026d,X      ; 8d 6d 02
a788: LDA  #$01         ; a9 01
a78a: STA  $026a,X      ; 8d 6a 02
a78d: STA  $026e,X      ; 8d 6e 02
a790: RTS               ; 60 
a791: CMP  $13b3,X      ; cd b3 13
a794: BNE  $a797        ; d0 01
a796: RTS               ; 60 
a797: STA  $13b3,X      ; 8d b3 13
a79a: PHA               ; 48 
a79b: ASL               ; 0a 
a79c: ASL               ; 0a 
a79d: ASL               ; 0a 
a79e: STA  $4b          ; 85 4b
a7a0: LDA  $09          ; a5 09
a7a2: AND  #$c7         ; 29 c7
a7a4: ORA  $4b          ; 05 4b
a7a6: STA  $09          ; 85 09
a7a8: PLA               ; 68 
a7a9: ASL               ; 0a 
a7aa: TAX               ; aa 
a7ab: LDA  #$00         ; a9 00
a7ad: STA  $0480,X      ; 8d 80 04
a7b0: STA  $0481,X      ; 8d 81 04
a7b3: JSR  $a567        ; 20 67 a5
a7b6: LDA  ($80)        ; b2 80
a7b8: AND  $03b2,X      ; 3d b2 03
a7bb: .byte $3b   ;?
a7bc: LDA  ($c0)        ; b2 c0
a7be: DEA               ; 3a 
a7bf: LDA  $38          ; a5 38
a7c1: LDA  #$00         ; a9 00
a7c3: STA  $0454,X      ; 8d 54 04
a7c6: LDA  $ebb0,X      ; bd b0 eb
a7c9: STA  $0455,X      ; 8d 55 04
a7cc: LDA  $ebb1,X      ; bd b1 eb
a7cf: STA  $0457,X      ; 8d 57 04
a7d2: CMP  ($40)        ; d2 40
a7d4: TRB  $04          ; 54 04
a7d6: RTS               ; 60 
a7d7: .byte $0f   ;?
a7d8: TYA               ; 98 
a7d9: .byte $42   ;?
a7da: LDA  $1265,X      ; ad 65 12
a7dd: CMP  #$ff         ; c9 ff
a7df: BEQ  $a847        ; f0 66
a7e1: LDA  ($14)        ; b2 14
a7e3: RTI               ; 40 
a7e4: LDA  ($00)        ; b2 00
a7e6: EOR  ($20,X)      ; 41 20
a7e8: .byte $a7   ;?
a7e9: LDX  $50c9        ; ae c9 50
a7ec: BEQ  $a808        ; f0 1a
a7ee: LDA  ($71)        ; b2 71
a7f0: RTI               ; 40 
a7f1: LDA  ($01)        ; b2 01
a7f3: EOR  ($20,X)      ; 41 20
a7f5: .byte $a7   ;?
a7f6: LDX  $50c9        ; ae c9 50
a7f9: BEQ  $a808        ; f0 0d
a7fb: LDA  ($80)        ; b2 80
a7fd: RTI               ; 40 
a7fe: LDA  ($01)        ; b2 01
a800: EOR  ($20,X)      ; 41 20
a802: .byte $a7   ;?
a803: LDX  $50c9        ; ae c9 50
a806: BNE  $a81c        ; d0 14
a808: .byte $b7   ;?
a809: .byte $9b   ;?
a80a: INX               ; e8 
a80b: JSR  $eb9e        ; 20 9e eb
a80e: PHA               ; 48 
a80f: INX               ; e8 
a810: JSR  $eb9e        ; 20 9e eb
a813: STA  $41          ; 85 41
a815: PLA               ; 68 
a816: STA  $40          ; 85 40
a818: .byte $78   ;?
a819: JMP  ($b137)      ; 4c 37 b1
a81c: LDA  ($14)        ; b2 14
a81e: RTI               ; 40 
a81f: LDA  ($00)        ; b2 00
a821: EOR  ($20,X)      ; 41 20
a823: SBC  $ea          ; e5 ea
a825: CMP  #$50         ; c9 50
a827: BEQ  $a843        ; f0 1a
a829: LDA  ($71)        ; b2 71
a82b: RTI               ; 40 
a82c: LDA  ($01)        ; b2 01
a82e: EOR  ($20,X)      ; 41 20
a830: SBC  $ea          ; e5 ea
a832: CMP  #$50         ; c9 50
a834: BEQ  $a843        ; f0 0d
a836: LDA  ($80)        ; b2 80
a838: RTI               ; 40 
a839: LDA  ($01)        ; b2 01
a83b: EOR  ($20,X)      ; 41 20
a83d: SBC  $ea          ; e5 ea
a83f: CMP  #$50         ; c9 50
a841: BNE  $a847        ; d0 04
a843: .byte $f7   ;?
a844: TYA               ; 98 
a845: BRA  $a808        ; 80 c1
a847: .byte $37   ;?
a848: .byte $9b   ;?
a849: RTS               ; 60 
a84a: INC  $40,X        ; e6 40
a84c: BNE  $a850        ; d0 02
a84e: INC  $41,X        ; e6 41
a850: RTS               ; 60 
a851: INC  $42,X        ; e6 42
a853: BNE  $a857        ; d0 02
a855: INC  $43,X        ; e6 43
a857: RTS               ; 60 
a858: INC  $46,X        ; e6 46
a85a: BNE  $a85e        ; d0 02
a85c: INC  $47,X        ; e6 47
a85e: RTS               ; 60 
a85f: NOP               ; ea 
a860: JSR  $13b4        ; 20 b4 13
a863: CLI               ; 58 
a864: LDA  $04ea,X      ; ad ea 04
a867: ASL               ; 0a 
a868: ASL               ; 0a 
a869: STA  $4b          ; 85 4b
a86b: LDA  $0891,X      ; ad 91 08
a86e: AND  #$c3         ; 29 c3
a870: ORA  $4b          ; 05 4b
a872: STA  $0891,X      ; 8d 91 08
a875: .byte $e2   ;?
a876: ORA  ($0d,X)      ; 01 0d
a878: BPL  $a883        ; 10 09
a87a: .byte $c2   ;?
a87b: BPL  $a87e        ; 10 01
a87d: ORA  $28a2,X      ; 0d a2 28
a880: JSR  $a4c5        ; 20 c5 a4
a883: .byte $af   ;?
a884: .byte $a3   ;?
a885: ASL  $02d2        ; 0e d2 02
a888: .byte $02   ;?
a889: ORA  $02d2,X      ; 0d d2 02
a88c: ORA  ($0d,X)      ; 01 0d
a88e: CMP  ($02)        ; d2 02
a890: .byte $03   ;?
a891: ORA  $93a7,X      ; 0d a7 93
a894: JSR  $a58e        ; 20 8e a5
a897: .byte $17   ;?
a898: ORA  ($27,X)      ; 01 27
a89a: .byte $99   ;?
a89b: JSR  $b7c0        ; 20 c0 b7
a89e: JSR  $e9c3        ; 20 c3 e9
a8a1: .byte $17   ;?
a8a2: ORA  ($d2,X)      ; 01 d2
a8a4: RTI               ; 40 
a8a5: PHX               ; da 
a8a6: TSB  $20          ; 04 20
a8a8: .byte $b7   ;?
a8a9: .byte $13   ;?
a8aa: JSR  $13ba        ; 20 ba 13
a8ad: JSR  $baa1        ; 20 a1 ba
a8b0: JSR  $b32e        ; 20 2e b3
a8b3: JSR  $b933        ; 20 33 b9
a8b6: JSR  $a8ef        ; 20 ef a8
a8b9: .byte $0f   ;?
a8ba: .byte $93   ;?
a8bb: ASL  $20,X        ; 06 20
a8bd: INC  $13,X        ; f6 13
a8bf: JSR  $123b        ; 20 3b 12
a8c2: SBC  ($00)        ; f2 00
a8c4: BPL  $a846        ; 10 80
a8c6: .byte $e3   ;?
a8c7: .byte $6f   ;?
a8c8: .byte $93   ;?
a8c9: PHP               ; 08 
a8ca: JSR  $13f9        ; 20 f9 13
a8cd: JSR  $1247        ; 20 47 12
a8d0: BRA  $a8aa        ; 80 d8
a8d2: JSR  $a960        ; 20 60 a9
a8d5: JSR  $ea95        ; 20 95 ea
a8d8: JSR  $e67d        ; 20 7d e6
a8db: JSR  $e75c        ; 20 5c e7
a8de: .byte $1f   ;?
a8df: .byte $93   ;?
a8e0: ASL  $20,X        ; 06 20
a8e2: LSR  $14,X        ; 56 14
a8e4: JSR  $1253        ; 20 53 12
a8e7: JSR  $e7af        ; 20 af e7
a8ea: JSR  $ba3f        ; 20 3f ba
a8ed: BRA  $a8aa        ; 80 bb
a8ef: .byte $22   ;?
a8f0: LDA  $40          ; a5 40
a8f2: PHA               ; 48 
a8f3: LDA  $41          ; a5 41
a8f5: PHA               ; 48 
a8f6: .byte $4f   ;?
a8f7: .byte $9f   ;?
a8f8: ORA  $47          ; 05 47
a8fa: .byte $9f   ;?
a8fb: JSR  $bdc1        ; 20 c1 bd
a8fe: .byte $7f   ;?
a8ff: ORA  ($2d,X)      ; 01 2d
a901: .byte $2f   ;?
a902: .byte $93   ;?
a903: ASL  $20,X        ; 06 20
a905: ORA  ($14),Y      ; 11 14
a907: JSR  $124a        ; 20 4a 12
a90a: .byte $3f   ;?
a90b: .byte $9f   ;?
a90c: ASL  $20,X        ; 06 20
a90e: TRB  $14          ; 14 14
a910: JSR  $124d        ; 20 4d 12
a913: .byte $7f   ;?
a914: STA  $2003,X      ; 9d 03 20
a917: ADC  $7fd3,X      ; 6d d3 7f
a91a: STZ  ($2003)      ; 9c 03 20
a91d: STA  $20d4,X      ; 8d d4 20
a920: STZ  $d5          ; 64 d5
a922: .byte $5f   ;?
a923: .byte $9f   ;?
a924: .byte $03   ;?
a925: JSR  $d459        ; 20 59 d4
a928: .byte $4f   ;?
a929: .byte $a3   ;?
a92a: .byte $03   ;?
a92b: JSR  $d539        ; 20 39 d5
a92e: PLA               ; 68 
a92f: STA  $41          ; 85 41
a931: PLA               ; 68 
a932: STA  $40          ; 85 40
a934: AND  ($60)        ; 32 60
a936: .byte $22   ;?
a937: LDA  $40          ; a5 40
a939: PHA               ; 48 
a93a: LDA  $41          ; a5 41
a93c: PHA               ; 48 
a93d: .byte $7f   ;?
a93e: ORA  ($18,X)      ; 01 18
a940: .byte $2f   ;?
a941: .byte $93   ;?
a942: ASL  $20,X        ; 06 20
a944: ORA  ($14),Y      ; 11 14
a946: JSR  $124a        ; 20 4a 12
a949: .byte $3f   ;?
a94a: .byte $9f   ;?
a94b: ASL  $20,X        ; 06 20
a94d: TRB  $14          ; 14 14
a94f: JSR  $124d        ; 20 4d 12
a952: .byte $7f   ;?
a953: STZ  ($2003)      ; 9c 03 20
a956: STA  $68d4,X      ; 8d d4 68
a959: STA  $41          ; 85 41
a95b: PLA               ; 68 
a95c: STA  $40          ; 85 40
a95e: AND  ($60)        ; 32 60
a960: JSR  $1483        ; 20 83 14
a963: .byte $2f   ;?
a964: .byte $a3   ;?
a965: INA               ; 1a 
a966: LDA  #$13         ; a9 13
a968: STA  $08b7,X      ; 8d b7 08
a96b: LDA  #$03         ; a9 03
a96d: STA  $08b8,X      ; 8d b8 08
a970: SBC  ($50)        ; f2 50
a972: .byte $0f   ;?
a973: ORA  ($0b,X)      ; 01 0b
a975: LDX  #$0d         ; a2 0d
a977: LDA  $ebc7,X      ; bd c7 eb
a97a: STA  $0f50,X      ; 9d 50 0f
a97d: DEX               ; ca 
a97e: BPL  $a977        ; 10 f7
a980: RTS               ; 60 
a981: JSR  $ae08        ; 20 08 ae
a984: LDA  #$a0         ; a9 a0
a986: JSR  $acff        ; 20 ff ac
a989: PHA               ; 48 
a98a: JSR  $ae2b        ; 20 2b ae
a98d: PHA               ; 48 
a98e: .byte $87   ;?
a98f: TYA               ; 98 
a990: PLA               ; 68 
a991: CLC               ; 18 
a992: PLA               ; 68 
a993: BEQ  $a9a1        ; f0 0c
a995: .byte $b7   ;?
a996: TYA               ; 98 
a997: .byte $a7   ;?
a998: TYA               ; 98 
a999: .byte $07   ;?
a99a: TYA               ; 98 
a99b: LDA  #$ff         ; a9 ff
a99d: STA  $1265,X      ; 8d 65 12
a9a0: SEC               ; 38 
a9a1: PHX               ; da 
a9a2: PHY               ; 5a 
a9a3: LDY  #$64         ; a0 64
a9a5: LDX  #$00         ; a2 00
a9a7: DEX               ; ca 
a9a8: BNE  $a9a7        ; d0 fd
a9aa: DEY               ; 88 
a9ab: BNE  $a9a5        ; d0 f8
a9ad: PLY               ; 7a 
a9ae: PLX               ; fa 
a9af: RTS               ; 60 
a9b0: LDA  ($00)        ; b2 00
a9b2: RTI               ; 40 
a9b3: LDA  ($00)        ; b2 00
a9b5: EOR  ($20,X)      ; 41 20
a9b7: .byte $a7   ;?
a9b8: LDX  $4cc9        ; ae c9 4c
a9bb: BNE  $a9cc        ; d0 0f
a9bd: LDA  ($02)        ; b2 02
a9bf: RTI               ; 40 
a9c0: LDA  ($00)        ; b2 00
a9c2: EOR  ($20,X)      ; 41 20
a9c4: .byte $a7   ;?
a9c5: LDX  $28c9        ; ae c9 28
a9c8: BNE  $a9cc        ; d0 02
a9ca: SEC               ; 38 
a9cb: RTS               ; 60 
a9cc: CLC               ; 18 
a9cd: RTS               ; 60 
a9ce: LDA  ($02)        ; b2 02
a9d0: RTI               ; 40 
a9d1: LDA  ($00)        ; b2 00
a9d3: EOR  ($a0,X)      ; 41 a0
a9d5: BRK               ; 00 
a9d6: JSR  $b106        ; 20 06 b1
a9d9: .byte $d9   ;?
a9da: CMP  $eb          ; d5 eb
a9dc: BNE  $a9e8        ; d0 0a
a9de: INY               ; c8 
a9df: CPY  #$0d         ; c0 0d
a9e1: BCC  $a9d6        ; 90 f3
a9e3: JSR  $ae56        ; 20 56 ae
a9e6: SEC               ; 38 
a9e7: RTS               ; 60 
a9e8: JSR  $ae56        ; 20 56 ae
a9eb: CLC               ; 18 
a9ec: RTS               ; 60 
a9ed: LDA  ($0f)        ; b2 0f
a9ef: RTI               ; 40 
a9f0: LDA  ($00)        ; b2 00
a9f2: EOR  ($20,X)      ; 41 20
a9f4: .byte $a7   ;?
a9f5: LDX  $b248        ; ae 48 b2
a9f8: BPL  $aa3a        ; 10 40
a9fa: LDA  ($00)        ; b2 00
a9fc: EOR  ($20,X)      ; 41 20
a9fe: .byte $a7   ;?
a9ff: LDX  $4185        ; ae 85 41
aa02: PLA               ; 68 
aa03: STA  $40          ; 85 40
aa05: LDA  #$00         ; a9 00
aa07: CMP  $40          ; c5 40
aa09: BNE  $aa0f        ; d0 04
aa0b: CMP  $41          ; c5 41
aa0d: BEQ  $aa3b        ; f0 2c
aa0f: JSR  $b106        ; 20 06 b1
aa12: STA  $42          ; 85 42
aa14: STA  $46          ; 85 46
aa16: JSR  $b106        ; 20 06 b1
aa19: STA  $43          ; 85 43
aa1b: STA  $47          ; 85 47
aa1d: JSR  $b106        ; 20 06 b1
aa20: STA  $44          ; 85 44
aa22: JSR  $b106        ; 20 06 b1
aa25: STA  $45          ; 85 45
aa27: JSR  $b106        ; 20 06 b1
aa2a: STA  ($42,X)      ; 81 42
aa2c: JSR  $a851        ; 20 51 a8
aa2f: LDA  $45          ; a5 45
aa31: CMP  $43          ; c5 43
aa33: BNE  $aa39        ; d0 04
aa35: LDA  $44          ; a5 44
aa37: CMP  $42          ; c5 42
aa39: BNE  $aa27        ; d0 ec
aa3b: JSR  $ae56        ; 20 56 ae
aa3e: JMP  ($0046)      ; 6c 46 00
aa41: LDY  #$06         ; a0 06
aa43: DEY               ; 88 
aa44: BNE  $aa48        ; d0 02
aa46: BRA  $aa9b        ; 80 53
aa48: PHA               ; 48 
aa49: .byte $b7   ;?
aa4a: TYA               ; 98 
aa4b: PLA               ; 68 
aa4c: JSR  $a9b0        ; 20 b0 a9
aa4f: BCS  $aa77        ; b0 26
aa51: PHA               ; 48 
aa52: .byte $37   ;?
aa53: TYA               ; 98 
aa54: PLA               ; 68 
aa55: JSR  $a9b0        ; 20 b0 a9
aa58: BCS  $aa5c        ; b0 02
aa5a: BRA  $aa43        ; 80 e7
aa5c: LDA  ($01)        ; b2 01
aa5e: RTI               ; 40 
aa5f: LDA  ($00)        ; b2 00
aa61: EOR  ($20,X)      ; 41 20
aa63: .byte $a7   ;?
aa64: LDX  $00c9        ; ae c9 00
aa67: BEQ  $aa98        ; f0 2f
aa69: CMP  #$01         ; c9 01
aa6b: BEQ  $aa98        ; f0 2b
aa6d: CMP  #$02         ; c9 02
aa6f: BEQ  $aa98        ; f0 27
aa71: CMP  #$03         ; c9 03
aa73: BEQ  $aa98        ; f0 23
aa75: BRA  $aa43        ; 80 cc
aa77: LDA  ($01)        ; b2 01
aa79: RTI               ; 40 
aa7a: LDA  ($00)        ; b2 00
aa7c: EOR  ($20,X)      ; 41 20
aa7e: .byte $a7   ;?
aa7f: LDX  $04c9        ; ae c9 04
aa82: BEQ  $aa98        ; f0 14
aa84: CMP  #$05         ; c9 05
aa86: BEQ  $aa98        ; f0 10
aa88: CMP  #$06         ; c9 06
aa8a: BEQ  $aa98        ; f0 0c
aa8c: CMP  #$07         ; c9 07
aa8e: BEQ  $aa98        ; f0 08
aa90: CMP  #$08         ; c9 08
aa92: BNE  $aa43        ; d0 af
aa94: PHA               ; 48 
aa95: .byte $a7   ;?
aa96: TYA               ; 98 
aa97: PLA               ; 68 
aa98: JMP  ($abfa)      ; 4c fa ab
aa9b: .byte $37   ;?
aa9c: TYA               ; 98 
aa9d: .byte $27   ;?
aa9e: TYA               ; 98 
aa9f: LDA  ($fd)        ; b2 fd
aaa1: .byte $42   ;?
aaa2: LDA  ($00)        ; b2 00
aaa4: .byte $43   ;?
aaa5: LDA  #$5a         ; a9 5a
aaa7: JSR  $aea1        ; 20 a1 ae
aaaa: LDA  ($fd)        ; b2 fd
aaac: RTI               ; 40 
aaad: LDA  ($00)        ; b2 00
aaaf: EOR  ($20,X)      ; 41 20
aab1: .byte $a7   ;?
aab2: LDX  $5ac9        ; ae c9 5a
aab5: BEQ  $aaba        ; f0 03
aab7: JMP  ($ab31)      ; 4c 31 ab
aaba: LDA  ($fd)        ; b2 fd
aabc: .byte $42   ;?
aabd: LDA  ($07)        ; b2 07
aabf: .byte $43   ;?
aac0: LDA  #$04         ; a9 04
aac2: JSR  $aea1        ; 20 a1 ae
aac5: LDA  ($fd)        ; b2 fd
aac7: .byte $42   ;?
aac8: LDA  ($03)        ; b2 03
aaca: .byte $43   ;?
aacb: LDA  #$03         ; a9 03
aacd: JSR  $aea1        ; 20 a1 ae
aad0: LDA  ($fd)        ; b2 fd
aad2: .byte $42   ;?
aad3: LDA  ($01)        ; b2 01
aad5: .byte $43   ;?
aad6: LDA  #$02         ; a9 02
aad8: JSR  $aea1        ; 20 a1 ae
aadb: LDA  ($fd)        ; b2 fd
aadd: .byte $42   ;?
aade: LDA  ($00)        ; b2 00
aae0: .byte $43   ;?
aae1: LDA  #$01         ; a9 01
aae3: JSR  $aea1        ; 20 a1 ae
aae6: LDA  ($fd)        ; b2 fd
aae8: RTI               ; 40 
aae9: LDA  ($00)        ; b2 00
aaeb: EOR  ($20,X)      ; 41 20
aaed: .byte $a7   ;?
aaee: LDX  $01c9        ; ae c9 01
aaf1: BEQ  $aaf6        ; f0 03
aaf3: JMP  ($abe0)      ; 4c e0 ab
aaf6: LDA  ($fd)        ; b2 fd
aaf8: RTI               ; 40 
aaf9: LDA  ($01)        ; b2 01
aafb: EOR  ($20,X)      ; 41 20
aafd: .byte $a7   ;?
aafe: LDX  $02c9        ; ae c9 02
ab01: BEQ  $ab08        ; f0 05
ab03: LDA  #$00         ; a9 00
ab05: JMP  ($abec)      ; 4c ec ab
ab08: LDA  ($fd)        ; b2 fd
ab0a: RTI               ; 40 
ab0b: LDA  ($03)        ; b2 03
ab0d: EOR  ($20,X)      ; 41 20
ab0f: .byte $a7   ;?
ab10: LDX  $03c9        ; ae c9 03
ab13: BEQ  $ab1a        ; f0 05
ab15: LDA  #$01         ; a9 01
ab17: JMP  ($abec)      ; 4c ec ab
ab1a: LDA  ($fd)        ; b2 fd
ab1c: RTI               ; 40 
ab1d: LDA  ($07)        ; b2 07
ab1f: EOR  ($20,X)      ; 41 20
ab21: .byte $a7   ;?
ab22: LDX  $04c9        ; ae c9 04
ab25: BEQ  $ab2c        ; f0 05
ab27: LDA  #$02         ; a9 02
ab29: JMP  ($abec)      ; 4c ec ab
ab2c: LDA  #$03         ; a9 03
ab2e: JMP  ($abec)      ; 4c ec ab
ab31: PHA               ; 48 
ab32: .byte $b7   ;?
ab33: TYA               ; 98 
ab34: PLA               ; 68 
ab35: LDA  ($fd)        ; b2 fd
ab37: .byte $42   ;?
ab38: LDA  ($0f)        ; b2 0f
ab3a: .byte $43   ;?
ab3b: LDA  #$5a         ; a9 5a
ab3d: JSR  $aea1        ; 20 a1 ae
ab40: LDA  ($fd)        ; b2 fd
ab42: RTI               ; 40 
ab43: LDA  ($0f)        ; b2 0f
ab45: EOR  ($20,X)      ; 41 20
ab47: .byte $a7   ;?
ab48: LDX  $5ac9        ; ae c9 5a
ab4b: BEQ  $ab50        ; f0 03
ab4d: JMP  ($abe0)      ; 4c e0 ab
ab50: .byte $a7   ;?
ab51: TYA               ; 98 
ab52: LDA  ($fd)        ; b2 fd
ab54: .byte $42   ;?
ab55: LDA  ($ff)        ; b2 ff
ab57: .byte $43   ;?
ab58: LDA  #$09         ; a9 09
ab5a: JSR  $aea1        ; 20 a1 ae
ab5d: LDA  ($fd)        ; b2 fd
ab5f: .byte $42   ;?
ab60: LDA  ($7f)        ; b2 7f
ab62: .byte $43   ;?
ab63: LDA  #$08         ; a9 08
ab65: JSR  $aea1        ; 20 a1 ae
ab68: LDA  ($fd)        ; b2 fd
ab6a: .byte $42   ;?
ab6b: LDA  ($3f)        ; b2 3f
ab6d: .byte $43   ;?
ab6e: LDA  #$07         ; a9 07
ab70: JSR  $aea1        ; 20 a1 ae
ab73: LDA  ($fd)        ; b2 fd
ab75: .byte $42   ;?
ab76: LDA  ($1f)        ; b2 1f
ab78: .byte $43   ;?
ab79: LDA  #$06         ; a9 06
ab7b: JSR  $aea1        ; 20 a1 ae
ab7e: LDA  ($fd)        ; b2 fd
ab80: .byte $42   ;?
ab81: LDA  ($0f)        ; b2 0f
ab83: .byte $43   ;?
ab84: LDA  #$05         ; a9 05
ab86: JSR  $aea1        ; 20 a1 ae
ab89: LDA  ($fd)        ; b2 fd
ab8b: RTI               ; 40 
ab8c: LDA  ($0f)        ; b2 0f
ab8e: EOR  ($20,X)      ; 41 20
ab90: .byte $a7   ;?
ab91: LDX  $05c9        ; ae c9 05
ab94: BEQ  $ab98        ; f0 02
ab96: BRA  $abe0        ; 80 48
ab98: LDA  ($fd)        ; b2 fd
ab9a: RTI               ; 40 
ab9b: LDA  ($1f)        ; b2 1f
ab9d: EOR  ($20,X)      ; 41 20
ab9f: .byte $a7   ;?
aba0: LDX  $06c9        ; ae c9 06
aba3: BEQ  $aba9        ; f0 04
aba5: LDA  #$04         ; a9 04
aba7: BRA  $abec        ; 80 43
aba9: LDA  ($fd)        ; b2 fd
abab: RTI               ; 40 
abac: LDA  ($3f)        ; b2 3f
abae: EOR  ($20,X)      ; 41 20
abb0: .byte $a7   ;?
abb1: LDX  $07c9        ; ae c9 07
abb4: BEQ  $abba        ; f0 04
abb6: LDA  #$05         ; a9 05
abb8: BRA  $abec        ; 80 32
abba: LDA  ($fd)        ; b2 fd
abbc: RTI               ; 40 
abbd: LDA  ($7f)        ; b2 7f
abbf: EOR  ($20,X)      ; 41 20
abc1: .byte $a7   ;?
abc2: LDX  $08c9        ; ae c9 08
abc5: BEQ  $abcb        ; f0 04
abc7: LDA  #$06         ; a9 06
abc9: BRA  $abec        ; 80 21
abcb: LDA  ($fd)        ; b2 fd
abcd: RTI               ; 40 
abce: LDA  ($ff)        ; b2 ff
abd0: EOR  ($20,X)      ; 41 20
abd2: .byte $a7   ;?
abd3: LDX  $09c9        ; ae c9 09
abd6: BEQ  $abdc        ; f0 04
abd8: LDA  #$07         ; a9 07
abda: BRA  $abec        ; 80 10
abdc: LDA  #$08         ; a9 08
abde: BRA  $abec        ; 80 0c
abe0: .byte $07   ;?
abe1: TYA               ; 98 
abe2: .byte $b7   ;?
abe3: TYA               ; 98 
abe4: .byte $a7   ;?
abe5: TYA               ; 98 
abe6: LDA  #$ff         ; a9 ff
abe8: STA  $1265,X      ; 8d 65 12
abeb: RTS               ; 60 
abec: STA  $1265,X      ; 8d 65 12
abef: PHA               ; 48 
abf0: LDA  ($01)        ; b2 01
abf2: .byte $42   ;?
abf3: LDA  ($00)        ; b2 00
abf5: .byte $43   ;?
abf6: JSR  $aea1        ; 20 a1 ae
abf9: PLA               ; 68 
abfa: STA  $1265,X      ; 8d 65 12
abfd: CMP  #$08         ; c9 08
abff: BEQ  $ac03        ; f0 02
ac01: .byte $27   ;?
ac02: TYA               ; 98 
ac03: RTS               ; 60 
ac04: JSR  $ae08        ; 20 08 ae
ac07: PHA               ; 48 
ac08: .byte $e7   ;?
ac09: ORA  $68          ; 05 68
ac0b: PHA               ; 48 
ac0c: .byte $e7   ;?
ac0d: ORA  ($68,X)      ; 01 68
ac0f: LDY  #$09         ; a0 09
ac11: JSR  $ae73        ; 20 73 ae
ac14: DEY               ; 88 
ac15: BNE  $ac11        ; d0 fa
ac17: JSR  $ae08        ; 20 08 ae
ac1a: JMP  ($ae2b)      ; 4c 2b ae
ac1d: PHY               ; 5a 
ac1e: JSR  $ac28        ; 20 28 ac
ac21: JSR  $ae4c        ; 20 4c ae
ac24: PLY               ; 7a 
ac25: AND  #$ff         ; 29 ff
ac27: RTS               ; 60 
ac28: JSR  $ae08        ; 20 08 ae
ac2b: JSR  $ada8        ; 20 a8 ad
ac2e: AND  #$fe         ; 29 fe
ac30: JSR  $acff        ; 20 ff ac
ac33: TXA               ; 8a 
ac34: STZ  $40          ; 64 40
ac36: .byte $3f   ;?
ac37: TYA               ; 98 
ac38: ASL               ; 0a 
ac39: LDA  #$00         ; a9 00
ac3b: ADC  $41          ; 65 41
ac3d: JSR  $acff        ; 20 ff ac
ac40: TXA               ; 8a 
ac41: STZ  $40          ; 64 40
ac43: JSR  $acff        ; 20 ff ac
ac46: JSR  $ae08        ; 20 08 ae
ac49: JSR  $ada8        ; 20 a8 ad
ac4c: JSR  $acff        ; 20 ff ac
ac4f: JSR  $ad3a        ; 20 3a ad
ac52: RTS               ; 60 
ac53: PHY               ; 5a 
ac54: JSR  $ac28        ; 20 28 ac
ac57: PLY               ; 7a 
ac58: AND  #$ff         ; 29 ff
ac5a: RTS               ; 60 
ac5b: STY  $46          ; 84 46
ac5d: STA  $47          ; 85 47
ac5f: LDA  $44          ; a5 44
ac61: PHA               ; 48 
ac62: LDA  $40          ; a5 40
ac64: PHA               ; 48 
ac65: LDA  $41          ; a5 41
ac67: PHA               ; 48 
ac68: LDA  $42          ; a5 42
ac6a: STA  $40          ; 85 40
ac6c: LDA  $43          ; a5 43
ac6e: STA  $41          ; 85 41
ac70: JSR  $ac1d        ; 20 1d ac
ac73: CMP  $47          ; c5 47
ac75: BEQ  $ac7c        ; f0 05
ac77: LDA  $47          ; a5 47
ac79: JSR  $ac92        ; 20 92 ac
ac7c: LDA  $40          ; a5 40
ac7e: STA  $42          ; 85 42
ac80: LDA  $41          ; a5 41
ac82: STA  $43          ; 85 43
ac84: PLA               ; 68 
ac85: STA  $41          ; 85 41
ac87: PLA               ; 68 
ac88: STA  $40          ; 85 40
ac8a: PLA               ; 68 
ac8b: STA  $44          ; 85 44
ac8d: LDA  $47          ; a5 47
ac8f: LDY  #$00         ; a0 00
ac91: RTS               ; 60 
ac92: PHA               ; 48 
ac93: LDA  #$a0         ; a9 a0
ac95: STA  $1266,X      ; 8d 66 12
ac98: JSR  $ae08        ; 20 08 ae
ac9b: JSR  $ad63        ; 20 63 ad
ac9e: JSR  $acff        ; 20 ff ac
aca1: TXA               ; 8a 
aca2: STZ  $42          ; 64 42
aca4: .byte $3f   ;?
aca5: TYA               ; 98 
aca6: ASL               ; 0a 
aca7: LDA  #$00         ; a9 00
aca9: ADC  $43          ; 65 43
acab: JSR  $acff        ; 20 ff ac
acae: TXA               ; 8a 
acaf: STZ  $42          ; 64 42
acb1: JSR  $acff        ; 20 ff ac
acb4: PLA               ; 68 
acb5: JSR  $acff        ; 20 ff ac
acb8: .byte $8f   ;?
acb9: TXS               ; 9a 
acba: .byte $03   ;?
acbb: JSR  $acf0        ; 20 f0 ac
acbe: RTS               ; 60 
acbf: PHX               ; da 
acc0: .byte $8f   ;?
acc1: TXS               ; 9a 
acc2: .byte $0b   ;?
acc3: PHA               ; 48 
acc4: .byte $87   ;?
acc5: TXS               ; 9a 
acc6: PLA               ; 68 
acc7: LDX  #$00         ; a2 00
acc9: JSR  $ac92        ; 20 92 ac
accc: BRA  $acd1        ; 80 03
acce: JSR  $acff        ; 20 ff ac
acd1: INC  $42,X        ; e6 42
acd3: LDA  $42          ; a5 42
acd5: LDY  ($1265)      ; ac 65 12
acd8: .byte $39   ;?
acd9: .byte $e2   ;?
acda: SBC               ; eb 
acdb: BNE  $acee        ; d0 11
acdd: JSR  $acf0        ; 20 f0 ac
ace0: LDA  $42          ; a5 42
ace2: BNE  $acee        ; d0 0a
ace4: INC  $43,X        ; e6 43
ace6: BNE  $acee        ; d0 06
ace8: LDA  $98          ; a5 98
acea: EOR  #$02         ; 49 02
acec: STA  $98          ; 85 98
acee: PLX               ; fa 
acef: RTS               ; 60 
acf0: JSR  $ae2b        ; 20 2b ae
acf3: PHA               ; 48 
acf4: .byte $07   ;?
acf5: TXS               ; 9a 
acf6: PLA               ; 68 
acf7: PHX               ; da 
acf8: LDX  #$06         ; a2 06
acfa: JSR  $a4eb        ; 20 eb a4
acfd: PLX               ; fa 
acfe: RTS               ; 60 
acff: PHA               ; 48 
ad00: .byte $e7   ;?
ad01: ORA  $68          ; 05 68
ad03: LDY  #$08         ; a0 08
ad05: ROL               ; 2a 
ad06: BCC  $ad0e        ; 90 06
ad08: PHA               ; 48 
ad09: .byte $e7   ;?
ad0a: ORA  ($68,X)      ; 01 68
ad0c: BRA  $ad12        ; 80 04
ad0e: PHA               ; 48 
ad0f: .byte $67   ;?
ad10: ORA  ($68,X)      ; 01 68
ad12: JSR  $ae73        ; 20 73 ae
ad15: DEY               ; 88 
ad16: BNE  $ad05        ; d0 ed
ad18: ROL               ; 2a 
ad19: PHA               ; 48 
ad1a: .byte $47   ;?
ad1b: BRK               ; 00 
ad1c: PLA               ; 68 
ad1d: PHA               ; 48 
ad1e: .byte $67   ;?
ad1f: ORA  $68          ; 05 68
ad21: LDA  #$00         ; a9 00
ad23: JSR  $ae82        ; 20 82 ae
ad26: PHA               ; 48 
ad27: .byte $c7   ;?
ad28: BRK               ; 00 
ad29: PLA               ; 68 
ad2a: JSR  $ae82        ; 20 82 ae
ad2d: .byte $6f   ;?
ad2e: ORA  ($02,X)      ; 01 02
ad30: LDA  #$ff         ; a9 ff
ad32: PHA               ; 48 
ad33: .byte $47   ;?
ad34: BRK               ; 00 
ad35: PLA               ; 68 
ad36: JSR  $ae95        ; 20 95 ae
ad39: RTS               ; 60 
ad3a: PHA               ; 48 
ad3b: .byte $67   ;?
ad3c: ORA  $68          ; 05 68
ad3e: PHY               ; 5a 
ad3f: LDY  #$08         ; a0 08
ad41: LDA  #$00         ; a9 00
ad43: PHA               ; 48 
ad44: .byte $47   ;?
ad45: BRK               ; 00 
ad46: PLA               ; 68 
ad47: JSR  $ae82        ; 20 82 ae
ad4a: PHA               ; 48 
ad4b: .byte $c7   ;?
ad4c: BRK               ; 00 
ad4d: PLA               ; 68 
ad4e: JSR  $ae82        ; 20 82 ae
ad51: .byte $ef   ;?
ad52: ORA  ($04,X)      ; 01 04
ad54: CLC               ; 18 
ad55: ROL               ; 2a 
ad56: BRA  $ad5a        ; 80 02
ad58: SEC               ; 38 
ad59: ROL               ; 2a 
ad5a: DEY               ; 88 
ad5b: BNE  $ad43        ; d0 e6
ad5d: PHA               ; 48 
ad5e: .byte $47   ;?
ad5f: BRK               ; 00 
ad60: PLA               ; 68 
ad61: PLY               ; 7a 
ad62: RTS               ; 60 
ad63: .byte $bf   ;?
ad64: TYA               ; 98 
ad65: ORA  ($8a)        ; 12 8a
ad67: STZ  $42          ; 64 42
ad69: LDA  #$00         ; a9 00
ad6b: ADC  $43          ; 65 43
ad6d: AND  #$07         ; 29 07
ad6f: CLC               ; 18 
ad70: ROL               ; 2a 
ad71: ORA  $1266,X      ; 0d 66 12
ad74: STA  $1266,X      ; 8d 66 12
ad77: RTS               ; 60 
ad78: .byte $af   ;?
ad79: TYA               ; 98 
ad7a: ASL  $20,X        ; 06 20
ad7c: .byte $87   ;?
ad7d: LDA  $a009,X      ; ad 09 a0
ad80: RTS               ; 60 
ad81: JSR  $ad94        ; 20 94 ad
ad84: ORA  #$a0         ; 09 a0
ad86: RTS               ; 60 
ad87: TXA               ; 8a 
ad88: STZ  $42          ; 64 42
ad8a: LDA  #$00         ; a9 00
ad8c: ADC  $43          ; 65 43
ad8e: ROL               ; 2a 
ad8f: ROL               ; 2a 
ad90: ROL               ; 2a 
ad91: AND  #$02         ; 29 02
ad93: RTS               ; 60 
ad94: TXA               ; 8a 
ad95: STZ  $42          ; 64 42
ad97: LDA  #$00         ; a9 00
ad99: ADC  $43          ; 65 43
ad9b: ROL               ; 2a 
ad9c: ROL               ; 2a 
ad9d: AND  #$02         ; 29 02
ad9f: STA  $48          ; 85 48
ada1: LDA  $98          ; a5 98
ada3: AND  #$02         ; 29 02
ada5: ORA  $48          ; 05 48
ada7: RTS               ; 60 
ada8: .byte $bf   ;?
ada9: TYA               ; 98 
adaa: ASL  $648a        ; 0e 8a 64
adad: RTI               ; 40 
adae: LDA  #$00         ; a9 00
adb0: ADC  $41          ; 65 41
adb2: AND  #$07         ; 29 07
adb4: CLC               ; 18 
adb5: ROL               ; 2a 
adb6: ORA  #$a1         ; 09 a1
adb8: RTS               ; 60 
adb9: .byte $af   ;?
adba: TYA               ; 98 
adbb: ASL  $20,X        ; 06 20
adbd: INY               ; c8 
adbe: LDA  $a109,X      ; ad 09 a1
adc1: RTS               ; 60 
adc2: JSR  $add5        ; 20 d5 ad
adc5: ORA  #$a1         ; 09 a1
adc7: RTS               ; 60 
adc8: TXA               ; 8a 
adc9: STZ  $40          ; 64 40
adcb: LDA  #$00         ; a9 00
adcd: ADC  $41          ; 65 41
adcf: ROL               ; 2a 
add0: ROL               ; 2a 
add1: ROL               ; 2a 
add2: AND  #$02         ; 29 02
add4: RTS               ; 60 
add5: TXA               ; 8a 
add6: STZ  $40          ; 64 40
add8: LDA  #$00         ; a9 00
adda: ADC  $41          ; 65 41
addc: ROL               ; 2a 
addd: ROL               ; 2a 
adde: AND  #$02         ; 29 02
ade0: STA  $48          ; 85 48
ade2: LDA  $98          ; a5 98
ade4: AND  #$02         ; 29 02
ade6: ORA  $48          ; 05 48
ade8: RTS               ; 60 
ade9: PHA               ; 48 
adea: .byte $67   ;?
adeb: ORA  $68          ; 05 68
aded: PHX               ; da 
adee: LDX  #$00         ; a2 00
adf0: .byte $ef   ;?
adf1: ORA  ($13,X)      ; 01 13
adf3: PHA               ; 48 
adf4: .byte $47   ;?
adf5: BRK               ; 00 
adf6: PLA               ; 68 
adf7: JSR  $ae82        ; 20 82 ae
adfa: PHA               ; 48 
adfb: .byte $c7   ;?
adfc: BRK               ; 00 
adfd: PLA               ; 68 
adfe: JSR  $ae82        ; 20 82 ae
ae01: INX               ; e8 
ae02: CPX  #$0b         ; e0 0b
ae04: BCC  $adf0        ; 90 ea
ae06: PLX               ; fa 
ae07: RTS               ; 60 
ae08: JSR  $ade9        ; 20 e9 ad
ae0b: PHA               ; 48 
ae0c: .byte $e7   ;?
ae0d: ORA  $68          ; 05 68
ae0f: PHA               ; 48 
ae10: .byte $e7   ;?
ae11: ORA  ($68,X)      ; 01 68
ae13: JSR  $ae95        ; 20 95 ae
ae16: PHA               ; 48 
ae17: .byte $c7   ;?
ae18: BRK               ; 00 
ae19: PLA               ; 68 
ae1a: JSR  $ae82        ; 20 82 ae
ae1d: PHA               ; 48 
ae1e: .byte $67   ;?
ae1f: ORA  ($68,X)      ; 01 68
ae21: JSR  $ae82        ; 20 82 ae
ae24: PHA               ; 48 
ae25: .byte $47   ;?
ae26: BRK               ; 00 
ae27: PLA               ; 68 
ae28: JMP  ($ae95)      ; 4c 95 ae
ae2b: PHA               ; 48 
ae2c: .byte $e7   ;?
ae2d: ORA  $68          ; 05 68
ae2f: PHA               ; 48 
ae30: .byte $67   ;?
ae31: ORA  ($68,X)      ; 01 68
ae33: JSR  $ae82        ; 20 82 ae
ae36: PHA               ; 48 
ae37: .byte $c7   ;?
ae38: BRK               ; 00 
ae39: PLA               ; 68 
ae3a: JSR  $ae82        ; 20 82 ae
ae3d: PHA               ; 48 
ae3e: .byte $e7   ;?
ae3f: ORA  ($68,X)      ; 01 68
ae41: JSR  $ae82        ; 20 82 ae
ae44: JSR  $ae95        ; 20 95 ae
ae47: PHA               ; 48 
ae48: .byte $e7   ;?
ae49: ORA  $68          ; 05 68
ae4b: RTS               ; 60 
ae4c: PHA               ; 48 
ae4d: .byte $57   ;?
ae4e: TYA               ; 98 
ae4f: PLA               ; 68 
ae50: JSR  $ae73        ; 20 73 ae
ae53: JMP  ($ae2b)      ; 4c 2b ae
ae56: .byte $5f   ;?
ae57: TYA               ; 98 
ae58: ASL  $20,X        ; 06 20
ae5a: DEA               ; 3a 
ae5b: LDA  $4c4c,X      ; ad 4c 4c
ae5e: LDX  $4860        ; ae 60 48
ae61: .byte $e7   ;?
ae62: ORA  $68          ; 05 68
ae64: PHA               ; 48 
ae65: .byte $67   ;?
ae66: ORA  ($68,X)      ; 01 68
ae68: JSR  $ae73        ; 20 73 ae
ae6b: JSR  $ae82        ; 20 82 ae
ae6e: PHA               ; 48 
ae6f: .byte $67   ;?
ae70: ORA  $68          ; 05 68
ae72: RTS               ; 60 
ae73: JSR  $ae82        ; 20 82 ae
ae76: PHA               ; 48 
ae77: .byte $c7   ;?
ae78: BRK               ; 00 
ae79: PLA               ; 68 
ae7a: JSR  $ae82        ; 20 82 ae
ae7d: PHA               ; 48 
ae7e: .byte $47   ;?
ae7f: BRK               ; 00 
ae80: PLA               ; 68 
ae81: RTS               ; 60 
ae82: .byte $cf   ;?
ae83: TYA               ; 98 
ae84: BPL  $aea6        ; 10 20
ae86: STA  $ae          ; 95 ae
ae88: JSR  $ae95        ; 20 95 ae
ae8b: JSR  $ae95        ; 20 95 ae
ae8e: JSR  $ae95        ; 20 95 ae
ae91: NOP               ; ea 
ae92: NOP               ; ea 
ae93: NOP               ; ea 
ae94: RTS               ; 60 
ae95: .byte $bf   ;?
ae96: ORA  #$08         ; 09 08
ae98: .byte $cb   ;?
ae99: .byte $db   ;?
ae9a: .byte $cb   ;?
ae9b: .byte $db   ;?
ae9c: NOP               ; ea 
ae9d: NOP               ; ea 
ae9e: NOP               ; ea 
ae9f: NOP               ; ea 
aea0: RTS               ; 60 
aea1: LDX  #$00         ; a2 00
aea3: JSR  $ac5b        ; 20 5b ac
aea6: RTS               ; 60 
aea7: LDX  #$00         ; a2 00
aea9: JSR  $ac1d        ; 20 1d ac
aeac: RTS               ; 60 
aead: .byte $47   ;?
aeae: TYA               ; 98 
aeaf: LDA  $1265,X      ; ad 65 12
aeb2: CMP  #$ff         ; c9 ff
aeb4: BNE  $aeb7        ; d0 01
aeb6: RTS               ; 60 
aeb7: LDY  #$05         ; a0 05
aeb9: JSR  $a9b0        ; 20 b0 a9
aebc: BCS  $aefa        ; b0 3c
aebe: DEY               ; 88 
aebf: BPL  $aeb9        ; 10 f8
aec1: LDA  ($00)        ; b2 00
aec3: .byte $42   ;?
aec4: LDA  ($00)        ; b2 00
aec6: .byte $43   ;?
aec7: LDA  #$4c         ; a9 4c
aec9: JSR  $aea1        ; 20 a1 ae
aecc: LDA  ($01)        ; b2 01
aece: .byte $42   ;?
aecf: LDA  ($00)        ; b2 00
aed1: .byte $43   ;?
aed2: LDA  $1265,X      ; ad 65 12
aed5: JSR  $aea1        ; 20 a1 ae
aed8: LDA  ($02)        ; b2 02
aeda: .byte $42   ;?
aedb: LDA  ($00)        ; b2 00
aedd: .byte $43   ;?
aede: LDA  #$28         ; a9 28
aee0: JSR  $aea1        ; 20 a1 ae
aee3: JSR  $a851        ; 20 51 a8
aee6: LDA  #$00         ; a9 00
aee8: JSR  $aea1        ; 20 a1 ae
aeeb: JSR  $a851        ; 20 51 a8
aeee: LDA  $43          ; a5 43
aef0: CMP  #$00         ; c9 00
aef2: BNE  $aef8        ; d0 04
aef4: LDA  $42          ; a5 42
aef6: CMP  #$29         ; c9 29
aef8: BNE  $aee6        ; d0 ec
aefa: LDY  #$05         ; a0 05
aefc: LDA  ($29)        ; b2 29
aefe: RTI               ; 40 
aeff: LDA  ($00)        ; b2 00
af01: EOR  ($20,X)      ; 41 20
af03: .byte $a7   ;?
af04: LDX  $53c9        ; ae c9 53
af07: BNE  $af13        ; d0 0a
af09: JSR  $a84a        ; 20 4a a8
af0c: JSR  $aea7        ; 20 a7 ae
af0f: CMP  #$04         ; c9 04
af11: BEQ  $af40        ; f0 2d
af13: DEY               ; 88 
af14: BPL  $aefc        ; 10 e6
af16: LDA  ($29)        ; b2 29
af18: .byte $42   ;?
af19: LDA  ($00)        ; b2 00
af1b: .byte $43   ;?
af1c: LDA  #$53         ; a9 53
af1e: JSR  $aea1        ; 20 a1 ae
af21: JSR  $a851        ; 20 51 a8
af24: LDA  #$04         ; a9 04
af26: JSR  $aea1        ; 20 a1 ae
af29: JSR  $a851        ; 20 51 a8
af2c: LDA  #$00         ; a9 00
af2e: JSR  $aea1        ; 20 a1 ae
af31: JSR  $a851        ; 20 51 a8
af34: LDA  $43          ; a5 43
af36: CMP  #$00         ; c9 00
af38: BNE  $af3e        ; d0 04
af3a: LDA  $42          ; a5 42
af3c: CMP  #$42         ; c9 42
af3e: BNE  $af2c        ; d0 ec
af40: RTS               ; 60 
af41: LDA  #$1c         ; a9 1c
af43: STA  $1239,X      ; 8d 39 12
af46: LDA  #$b2         ; a9 b2
af48: STA  $123a,X      ; 8d 3a 12
af4b: LDX  #$1f         ; a2 1f
af4d: LDA  #$00         ; a9 00
af4f: STA  $0b00,X      ; 9d 00 0b
af52: DEX               ; ca 
af53: BPL  $af4f        ; 10 fa
af55: STA  $0240,X      ; 8d 40 02
af58: STA  $0241,X      ; 8d 41 02
af5b: .byte $87   ;?
af5c: ASL  $17,X        ; 06 17
af5e: ORA  ($87,X)      ; 01 87
af60: .byte $99   ;?
af61: LDA  ($eb)        ; b2 eb
af63: RTI               ; 40 
af64: LDA  ($eb)        ; b2 eb
af66: EOR  ($20,X)      ; 41 20
af68: STZ  ($37b4)      ; 9c b4 37
af6b: .byte $99   ;?
af6c: .byte $47   ;?
af6d: .byte $99   ;?
af6e: JSR  $afaf        ; 20 af af
af71: .byte $cf   ;?
af72: .byte $99   ;?
af73: ROL  $bf,X        ; 36 bf
af75: .byte $99   ;?
af76: .byte $33   ;?
af77: LDA  $0265,X      ; ad 65 02
af7a: STA  $47          ; 85 47
af7c: LDA  $0264,X      ; ad 64 02
af7f: STA  $46          ; 85 46
af81: JSR  $b089        ; 20 89 b0
af84: JSR  $afaf        ; 20 af af
af87: .byte $cf   ;?
af88: .byte $99   ;?
af89: JSR  $99bf        ; 20 bf 99
af8c: ORA  $20          ; 05 20
af8e: BIT  #$b0         ; 89 b0
af90: BRA  $af84        ; 80 f2
af92: JSR  $b1d7        ; 20 d7 b1
af95: JSR  $a5a1        ; 20 a1 a5
af98: BCC  $af9a        ; 90 00
af9a: .byte $df   ;?
af9b: .byte $97   ;?
af9c: ASL  $20,X        ; 06 20
af9e: .byte $c2   ;?
af9f: LDA  $6c          ; a5 6c
afa1: LSR  $00,X        ; 46 00
afa3: .byte $17   ;?
afa4: TYA               ; 98 
afa5: JSR  $a58e        ; 20 8e a5
afa8: CLC               ; 18 
afa9: RTS               ; 60 
afaa: JSR  $a58e        ; 20 8e a5
afad: SEC               ; 38 
afae: RTS               ; 60 
afaf: JSR  $afe8        ; 20 e8 af
afb2: JSR  $aff9        ; 20 f9 af
afb5: JSR  $b1e8        ; 20 e8 b1
afb8: .byte $cf   ;?
afb9: .byte $99   ;?
afba: BIT  ($33c9)      ; 2c c9 33
afbd: BEQ  $afc8        ; f0 09
afbf: CMP  #$37         ; c9 37
afc1: BEQ  $afc6        ; f0 03
afc3: .byte $c7   ;?
afc4: .byte $99   ;?
afc5: RTS               ; 60 
afc6: .byte $b7   ;?
afc7: .byte $99   ;?
afc8: JSR  $b00c        ; 20 0c b0
afcb: JSR  $b063        ; 20 63 b0
afce: JSR  $b063        ; 20 63 b0
afd1: BEQ  $afd8        ; f0 05
afd3: .byte $5f   ;?
afd4: .byte $97   ;?
afd5: .byte $02   ;?
afd6: .byte $97   ;?
afd7: TYA               ; 98 
afd8: JSR  $b01e        ; 20 1e b0
afdb: JSR  $b029        ; 20 29 b0
afde: .byte $cf   ;?
afdf: .byte $99   ;?
afe0: ASL  $20,X        ; 06 20
afe2: .byte $39   ;?
afe3: BCS  $b005        ; b0 20
afe5: .byte $53   ;?
afe6: BCS  $b048        ; b0 60
afe8: LDA  #$00         ; a9 00
afea: STA  $0262,X      ; 8d 62 02
afed: STA  $0263,X      ; 8d 63 02
aff0: STA  $0265,X      ; 8d 65 02
aff3: STA  $0264,X      ; 8d 64 02
aff6: .byte $17   ;?
aff7: TYA               ; 98 
aff8: RTS               ; 60 
aff9: .byte $cf   ;?
affa: .byte $99   ;?
affb: .byte $0f   ;?
affc: JSR  $b1e8        ; 20 e8 b1
afff: CMP  #$53         ; c9 53
b001: BEQ  $b00b        ; f0 08
b003: CMP  #$1b         ; c9 1b
b005: BEQ  $b00b        ; f0 04
b007: CMP  #$03         ; c9 03
b009: BNE  $aff9        ; d0 ee
b00b: RTS               ; 60 
b00c: JSR  $b063        ; 20 63 b0
b00f: CMP  #$05         ; c9 05
b011: BCS  $b017        ; b0 04
b013: .byte $c7   ;?
b014: .byte $99   ;?
b015: LDA  #$05         ; a9 05
b017: SEC               ; 38 
b018: SBC  #$05         ; e9 05
b01a: STA  $0262,X      ; 8d 62 02
b01d: RTS               ; 60 
b01e: JSR  $b063        ; 20 63 b0
b021: CLC               ; 18 
b022: ADC  $0265,X      ; 6d 65 02
b025: STA  $0265,X      ; 8d 65 02
b028: RTS               ; 60 
b029: JSR  $b063        ; 20 63 b0
b02c: CLC               ; 18 
b02d: ADC  $0264,X      ; 6d 64 02
b030: STA  $0264,X      ; 8d 64 02
b033: BCC  $b038        ; 90 03
b035: INC  $0265        ; ee 65 02
b038: RTS               ; 60 
b039: LDX  #$00         ; a2 00
b03b: LDY  ($0262)      ; ac 62 02
b03e: BEQ  $b052        ; f0 12
b040: JSR  $b063        ; 20 63 b0
b043: .byte $cf   ;?
b044: .byte $99   ;?
b045: TSB  ($429d)      ; 0c 9d 42
b048: .byte $02   ;?
b049: INX               ; e8 
b04a: DEY               ; 88 
b04b: BNE  $b040        ; d0 f3
b04d: LDA  #$2e         ; a9 2e
b04f: JSR  $a5a5        ; 20 a5 a5
b052: RTS               ; 60 
b053: JSR  $b063        ; 20 63 b0
b056: .byte $cf   ;?
b057: .byte $99   ;?
b058: ORA  #$ad         ; 09 ad
b05a: .byte $63   ;?
b05b: .byte $02   ;?
b05c: CMP  #$ff         ; c9 ff
b05e: BEQ  $b062        ; f0 02
b060: .byte $c7   ;?
b061: .byte $99   ;?
b062: RTS               ; 60 
b063: JSR  $b07d        ; 20 7d b0
b066: ASL               ; 0a 
b067: ASL               ; 0a 
b068: ASL               ; 0a 
b069: ASL               ; 0a 
b06a: STA  $0266,X      ; 8d 66 02
b06d: JSR  $b07d        ; 20 7d b0
b070: ORA  $0266,X      ; 0d 66 02
b073: PHA               ; 48 
b074: CLC               ; 18 
b075: ADC  $0263,X      ; 6d 63 02
b078: STA  $0263,X      ; 8d 63 02
b07b: PLA               ; 68 
b07c: RTS               ; 60 
b07d: JSR  $b1e8        ; 20 e8 b1
b080: CMP  #$41         ; c9 41
b082: BCC  $b086        ; 90 02
b084: SBC  #$08         ; e9 08
b086: SBC  #$2f         ; e9 2f
b088: RTS               ; 60 
b089: .byte $df   ;?
b08a: .byte $97   ;?
b08b: .byte $1b   ;?
b08c: LDX  #$00         ; a2 00
b08e: LDY  ($0262)      ; ac 62 02
b091: BEQ  $b0a6        ; f0 13
b093: LDA  $0265,X      ; ad 65 02
b096: STA  $41          ; 85 41
b098: LDA  $0264,X      ; ad 64 02
b09b: STA  $40          ; 85 40
b09d: LDA  $0242,X      ; bd 42 02
b0a0: STA  ($40),Y      ; 91 40
b0a2: INX               ; e8 
b0a3: DEY               ; 88 
b0a4: BNE  $b09d        ; d0 f7
b0a6: RTS               ; 60 
b0a7: .byte $47   ;?
b0a8: .byte $99   ;?
b0a9: LDA  $0265,X      ; ad 65 02
b0ac: STA  $43          ; 85 43
b0ae: LDA  $0264,X      ; ad 64 02
b0b1: STA  $42          ; 85 42
b0b3: LDX  #$00         ; a2 00
b0b5: LDA  $0262,X      ; ad 62 02
b0b8: BEQ  $b0a6        ; f0 ec
b0ba: LDA  $0265,X      ; ad 65 02
b0bd: STA  $43          ; 85 43
b0bf: LDA  $0264,X      ; ad 64 02
b0c2: STA  $42          ; 85 42
b0c4: LDA  $98          ; a5 98
b0c6: AND  #$02         ; 29 02
b0c8: STA  $49          ; 85 49
b0ca: LDA  $0242,X      ; bd 42 02
b0cd: JSR  $acbf        ; 20 bf ac
b0d0: INX               ; e8 
b0d1: CPX  ($0262)      ; ec 62 02
b0d4: BNE  $b0ca        ; d0 f4
b0d6: .byte $0f   ;?
b0d7: TXS               ; 9a 
b0d8: .byte $03   ;?
b0d9: JSR  $acf0        ; 20 f0 ac
b0dc: LDA  $98          ; a5 98
b0de: AND  #$fd         ; 29 fd
b0e0: ORA  $49          ; 05 49
b0e2: STA  $98          ; 85 98
b0e4: LDX  #$00         ; a2 00
b0e6: LDA  $0265,X      ; ad 65 02
b0e9: STA  $41          ; 85 41
b0eb: LDA  $0264,X      ; ad 64 02
b0ee: STA  $40          ; 85 40
b0f0: .byte $57   ;?
b0f1: TYA               ; 98 
b0f2: JSR  $b106        ; 20 06 b1
b0f5: CMP  $0242,X      ; dd 42 02
b0f8: BEQ  $b0fc        ; f0 02
b0fa: .byte $c7   ;?
b0fb: .byte $99   ;?
b0fc: INX               ; e8 
b0fd: DEC  $0262        ; ce 62 02
b100: BNE  $b0f2        ; d0 f0
b102: JSR  $ae56        ; 20 56 ae
b105: RTS               ; 60 
b106: .byte $df   ;?
b107: TYA               ; 98 
b108: .byte $0b   ;?
b109: .byte $d7   ;?
b10a: TYA               ; 98 
b10b: PHX               ; da 
b10c: LDX  #$00         ; a2 00
b10e: JSR  $ac53        ; 20 53 ac
b111: PLX               ; fa 
b112: BRA  $b117        ; 80 03
b114: JSR  $ad3a        ; 20 3a ad
b117: PHA               ; 48 
b118: INC  $40,X        ; e6 40
b11a: BNE  $b132        ; d0 16
b11c: INC  $41,X        ; e6 41
b11e: BNE  $b124        ; d0 04
b120: .byte $97   ;?
b121: TYA               ; 98 
b122: BRA  $b12d        ; 80 09
b124: .byte $af   ;?
b125: TYA               ; 98 
b126: .byte $0b   ;?
b127: LDY  #$80         ; a0 80
b129: CPY  $41          ; c4 41
b12b: BNE  $b132        ; d0 05
b12d: JSR  $ae4c        ; 20 4c ae
b130: PLA               ; 68 
b131: RTS               ; 60 
b132: JSR  $ae60        ; 20 60 ae
b135: PLA               ; 68 
b136: RTS               ; 60 
b137: LDX  #$00         ; a2 00
b139: .byte $57   ;?
b13a: TYA               ; 98 
b13b: JSR  $b140        ; 20 40 b1
b13e: BRA  $b13b        ; 80 fb
b140: CPX  #$16         ; e0 16
b142: BEQ  $b147        ; f0 03
b144: JSR  $eba7        ; 20 a7 eb
b147: JMP  ($b14a,X)    ; 7c 4a b1
b14a: .byte $62   ;?
b14b: LDA  ($82),Y      ; b1 82
b14d: LDA  ($89),Y      ; b1 89
b14f: LDA  ($90),Y      ; b1 90
b151: LDA  ($94),Y      ; b1 94
b153: LDA  ($9d),Y      ; b1 9d
b155: LDA  ($82),Y      ; b1 82
b157: LDA  ($89),Y      ; b1 89
b159: LDA  ($a1),Y      ; b1 a1
b15b: LDA  ($90),Y      ; b1 90
b15d: LDA  ($94),Y      ; b1 94
b15f: LDA  ($a8),Y      ; b1 a8
b161: LDA  ($48),Y      ; b1 48
b163: PHA               ; 48 
b164: LDA  #$00         ; a9 00
b166: STA  $47          ; 85 47
b168: STA  $46          ; 85 46
b16a: STA  $45          ; 85 45
b16c: STA  $44          ; 85 44
b16e: STA  $43          ; 85 43
b170: STA  $42          ; 85 42
b172: .byte $67   ;?
b173: .byte $99   ;?
b174: .byte $77   ;?
b175: .byte $99   ;?
b176: PLA               ; 68 
b177: AND  #$40         ; 29 40
b179: BEQ  $b17d        ; f0 02
b17b: .byte $e7   ;?
b17c: .byte $99   ;?
b17d: PLA               ; 68 
b17e: .byte $f7   ;?
b17f: .byte $99   ;?
b180: BRA  $b1a5        ; 80 23
b182: STA  $46          ; 85 46
b184: STA  $0264,X      ; 8d 64 02
b187: BRA  $b1a5        ; 80 1c
b189: STA  $47          ; 85 47
b18b: STA  $0265,X      ; 8d 65 02
b18e: BRA  $b1a5        ; 80 15
b190: STA  $42          ; 85 42
b192: BRA  $b1a5        ; 80 11
b194: STA  $43          ; 85 43
b196: .byte $7f   ;?
b197: .byte $99   ;?
b198: .byte $02   ;?
b199: LDX  #$14         ; a2 14
b19b: BRA  $b1a5        ; 80 08
b19d: STA  $45          ; 85 45
b19f: BRA  $b1a5        ; 80 04
b1a1: STA  $44          ; 85 44
b1a3: BRA  $b1a5        ; 80 00
b1a5: INX               ; e8 
b1a6: INX               ; e8 
b1a7: RTS               ; 60 
b1a8: JSR  $a851        ; 20 51 a8
b1ab: JSR  $eba7        ; 20 a7 eb
b1ae: STA  ($46,X)      ; 81 46
b1b0: JSR  $a858        ; 20 58 a8
b1b3: LDA  $47          ; a5 47
b1b5: CMP  $43          ; c5 43
b1b7: BNE  $b1bd        ; d0 04
b1b9: LDA  $46          ; a5 46
b1bb: CMP  $42          ; c5 42
b1bd: BNE  $b1ab        ; d0 ec
b1bf: .byte $ef   ;?
b1c0: .byte $99   ;?
b1c1: .byte $03   ;?
b1c2: LDX  #$00         ; a2 00
b1c4: RTS               ; 60 
b1c5: LDA  $0265,X      ; ad 65 02
b1c8: STA  $47          ; 85 47
b1ca: LDA  $0264,X      ; ad 64 02
b1cd: STA  $46          ; 85 46
b1cf: JSR  $b1d4        ; 20 d4 b1
b1d2: BRA  $b1c2        ; 80 ee
b1d4: JMP  ($0046)      ; 6c 46 00
b1d7: .byte $22   ;?
b1d8: LDY  #$c0         ; a0 c0
b1da: JSR  $b1e2        ; 20 e2 b1
b1dd: DEY               ; 88 
b1de: BNE  $b1da        ; d0 fa
b1e0: AND  ($60)        ; 32 60
b1e2: LDX  #$ff         ; a2 ff
b1e4: DEX               ; ca 
b1e5: BNE  $b1e4        ; d0 fd
b1e7: RTS               ; 60 
b1e8: BRA  $b1ea        ; 80 00
b1ea: PHX               ; da 
b1eb: LDX  $0240        ; ae 40 02
b1ee: CPX  ($0241)      ; ec 41 02
b1f1: BEQ  $b1eb        ; f0 f8
b1f3: JSR  $b1fd        ; 20 fd b1
b1f6: .byte $0f   ;?
b1f7: .byte $99   ;?
b1f8: .byte $02   ;?
b1f9: .byte $17   ;?
b1fa: ORA  ($fa,X)      ; 01 fa
b1fc: RTS               ; 60 
b1fd: LDX  $0240        ; ae 40 02
b200: LDA  $0200,X      ; bd 00 02
b203: PHA               ; 48 
b204: INX               ; e8 
b205: TXA               ; 8a 
b206: AND  #$3f         ; 29 3f
b208: STA  $0240,X      ; 8d 40 02
b20b: LDA  $0241,X      ; ad 41 02
b20e: SEC               ; 38 
b20f: SBC  $0240,X      ; ed 40 02
b212: AND  #$3f         ; 29 3f
b214: CMP  #$10         ; c9 10
b216: BCS  $b21a        ; b0 02
b218: .byte $87   ;?
b219: .byte $99   ;?
b21a: PLA               ; 68 
b21b: RTS               ; 60 
b21c: JSR  $b225        ; 20 25 b2
b21f: .byte $8f   ;?
b220: .byte $99   ;?
b221: .byte $02   ;?
b222: .byte $97   ;?
b223: ORA  ($60,X)      ; 01 60
b225: PHX               ; da 
b226: LDX  $0241        ; ae 41 02
b229: STA  $0200,X      ; 9d 00 02
b22c: INX               ; e8 
b22d: TXA               ; 8a 
b22e: AND  #$3f         ; 29 3f
b230: STA  $0241,X      ; 8d 41 02
b233: SEC               ; 38 
b234: SBC  $0240,X      ; ed 40 02
b237: AND  #$3f         ; 29 3f
b239: CMP  #$20         ; c9 20
b23b: BCC  $b23f        ; 90 02
b23d: .byte $07   ;?
b23e: .byte $99   ;?
b23f: PLX               ; fa 
b240: RTS               ; 60 
b241: .byte $db   ;?
b242: .byte $22   ;?
b243: .byte $ab   ;?
b244: ASL               ; 0a 
b245: TAX               ; aa 
b246: JSR  $b24b        ; 20 4b b2
b249: AND  ($03)        ; 32 03
b24b: JMP  ($b24e,X)    ; 7c 4e b2
b24e: ROR  $82b2        ; 7e b2 82
b251: LDA  ($af)        ; b2 af
b253: LDA  ($b3)        ; b2 b3
b255: LDA  ($e8)        ; b2 e8
b257: LDA  ($ec)        ; b2 ec
b259: LDA  ($86)        ; b2 86
b25b: LDA  ($8a)        ; b2 8a
b25d: LDA  ($8e)        ; b2 8e
b25f: LDA  ($92)        ; b2 92
b261: LDA  ($96)        ; b2 96
b263: LDA  ($9a)        ; b2 9a
b265: LDA  ($b7)        ; b2 b7
b267: LDA  ($bb)        ; b2 bb
b269: LDA  ($bf)        ; b2 bf
b26b: LDA  ($c3)        ; b2 c3
b26d: LDA  ($c7)        ; b2 c7
b26f: LDA  ($cb)        ; b2 cb
b271: LDA  ($f0)        ; b2 f0
b273: LDA  ($f4)        ; b2 f4
b275: LDA  ($f8)        ; b2 f8
b277: LDA  ($fc)        ; b2 fc
b279: LDA  ($00)        ; b2 00
b27b: .byte $b3   ;?
b27c: TSB  $b3          ; 04 b3
b27e: LDX  #$00         ; a2 00
b280: BRA  $b29e        ; 80 1c
b282: LDX  #$02         ; a2 02
b284: BRA  $b29e        ; 80 18
b286: LDX  #$08         ; a2 08
b288: BRA  $b29e        ; 80 14
b28a: LDX  #$0a         ; a2 0a
b28c: BRA  $b29e        ; 80 10
b28e: LDX  #$10         ; a2 10
b290: BRA  $b29e        ; 80 0c
b292: LDX  #$12         ; a2 12
b294: BRA  $b29e        ; 80 08
b296: LDX  #$18         ; a2 18
b298: BRA  $b29e        ; 80 04
b29a: LDX  #$1a         ; a2 1a
b29c: BRA  $b29e        ; 80 00
b29e: .byte $ab   ;?
b29f: STA  $0b00,X      ; 9d 00 0b
b2a2: .byte $ab   ;?
b2a3: STA  $0b01,X      ; 9d 01 0b
b2a6: .byte $ab   ;?
b2a7: STA  $0b04,X      ; 9d 04 0b
b2aa: .byte $ab   ;?
b2ab: STA  $0b05,X      ; 9d 05 0b
b2ae: RTS               ; 60 
b2af: LDX  #$00         ; a2 00
b2b1: BRA  $b2cf        ; 80 1c
b2b3: LDX  #$02         ; a2 02
b2b5: BRA  $b2cf        ; 80 18
b2b7: LDX  #$08         ; a2 08
b2b9: BRA  $b2cf        ; 80 14
b2bb: LDX  #$0a         ; a2 0a
b2bd: BRA  $b2cf        ; 80 10
b2bf: LDX  #$10         ; a2 10
b2c1: BRA  $b2cf        ; 80 0c
b2c3: LDX  #$12         ; a2 12
b2c5: BRA  $b2cf        ; 80 08
b2c7: LDX  #$18         ; a2 18
b2c9: BRA  $b2cf        ; 80 04
b2cb: LDX  #$1a         ; a2 1a
b2cd: BRA  $b2cf        ; 80 00
b2cf: LDA  $0b00,X      ; bd 00 0b
b2d2: STA  $027b,X      ; 9d 7b 02
b2d5: LDA  $0b01,X      ; bd 01 0b
b2d8: STA  $027c,X      ; 9d 7c 02
b2db: LDA  $0b04,X      ; bd 04 0b
b2de: STA  $027f,X      ; 9d 7f 02
b2e1: LDA  $0b05,X      ; bd 05 0b
b2e4: STA  $0280,X      ; 9d 80 02
b2e7: RTS               ; 60 
b2e8: LDX  #$00         ; a2 00
b2ea: BRA  $b308        ; 80 1c
b2ec: LDX  #$02         ; a2 02
b2ee: BRA  $b308        ; 80 18
b2f0: LDX  #$08         ; a2 08
b2f2: BRA  $b308        ; 80 14
b2f4: LDX  #$0a         ; a2 0a
b2f6: BRA  $b308        ; 80 10
b2f8: LDX  #$10         ; a2 10
b2fa: BRA  $b308        ; 80 0c
b2fc: LDX  #$12         ; a2 12
b2fe: BRA  $b308        ; 80 08
b300: LDX  #$18         ; a2 18
b302: BRA  $b308        ; 80 04
b304: LDX  #$1a         ; a2 1a
b306: BRA  $b308        ; 80 00
b308: LDA  $027b,X      ; bd 7b 02
b30b: STA  $0b00,X      ; 9d 00 0b
b30e: LDA  $027c,X      ; bd 7c 02
b311: STA  $0b01,X      ; 9d 01 0b
b314: LDA  $027f,X      ; bd 7f 02
b317: STA  $0b04,X      ; 9d 04 0b
b31a: LDA  $0280,X      ; bd 80 02
b31d: STA  $0b05,X      ; 9d 05 0b
b320: RTS               ; 60 
b321: .byte $ff   ;?
b322: TYA               ; 98 
b323: .byte $03   ;?
b324: JSR  $ae2b        ; 20 2b ae
b327: LDX  #$04         ; a2 04
b329: PLA               ; 68 
b32a: DEX               ; ca 
b32b: BNE  $b329        ; d0 fc
b32d: RTS               ; 60 
b32e: SBC  ($00)        ; f2 00
b330: TSB  $80          ; 04 80
b332: TRB  ($00f2)      ; 1c f2 00
b335: TSB  $40          ; 04 40
b337: ORA  ($60,X)      ; 01 60
b339: .byte $c2   ;?
b33a: BPL  $b33d        ; 10 01
b33c: ORA  $14a2,X      ; 0d a2 14
b33f: JSR  $a4c5        ; 20 c5 a4
b342: LDX  #$1f         ; a2 1f
b344: LDA  #$00         ; a9 00
b346: STA  $0b00,X      ; 9d 00 0b
b349: DEX               ; ca 
b34a: BPL  $b346        ; 10 fa
b34c: JMP  ($fffe)      ; 6c fe ff
b34f: JSR  $13f0        ; 20 f0 13
b352: LDA  $02fc,X      ; ad fc 02
b355: STA  $40          ; 85 40
b357: LDA  $02fd,X      ; ad fd 02
b35a: STA  $41          ; 85 41
b35c: LDX  #$00         ; a2 00
b35e: SBC  ($00)        ; f2 00
b360: TSB  $01          ; 04 01
b362: ROL  $e2,X        ; 26 e2
b364: BRK               ; 00 
b365: TSB  $02          ; 04 02
b367: .byte $1f   ;?
b368: .byte $17   ;?
b369: TYA               ; 98 
b36a: LDA  $02fe,X      ; ad fe 02
b36d: BEQ  $b371        ; f0 02
b36f: .byte $97   ;?
b370: TYA               ; 98 
b371: .byte $57   ;?
b372: TYA               ; 98 
b373: JSR  $b106        ; 20 06 b1
b376: STA  $0300,X      ; 9d 00 03
b379: CPX  ($02ff)      ; ec ff 02
b37c: BEQ  $b381        ; f0 03
b37e: INX               ; e8 
b37f: BRA  $b373        ; 80 f2
b381: JSR  $ae56        ; 20 56 ae
b384: JMP  ($b3b7)      ; 4c b7 b3
b387: BRA  $b3b7        ; 80 2e
b389: .byte $e2   ;?
b38a: BRK               ; 00 
b38b: TSB  $02          ; 04 02
b38d: AND  #$17         ; 29 17
b38f: TYA               ; 98 
b390: LDA  $02fe,X      ; ad fe 02
b393: BEQ  $b397        ; f0 02
b395: .byte $97   ;?
b396: TYA               ; 98 
b397: LDA  $02fd,X      ; ad fd 02
b39a: STA  $43          ; 85 43
b39c: LDA  $02fc,X      ; ad fc 02
b39f: STA  $42          ; 85 42
b3a1: LDA  $0300,X      ; bd 00 03
b3a4: JSR  $acbf        ; 20 bf ac
b3a7: CPX  ($02ff)      ; ec ff 02
b3aa: BEQ  $b3af        ; f0 03
b3ac: INX               ; e8 
b3ad: BRA  $b3a1        ; 80 f2
b3af: .byte $0f   ;?
b3b0: TXS               ; 9a 
b3b1: ORA  $20          ; 05 20
b3b3: BEQ  $b361        ; f0 ac
b3b5: BRA  $b3b7        ; 80 00
b3b7: .byte $17   ;?
b3b8: TYA               ; 98 
b3b9: .byte $c2   ;?
b3ba: BRA  $b3bc        ; 80 00
b3bc: TSB  $60          ; 04 60
b3be: AND  #$7f         ; 29 7f
b3c0: CMP  #$0d         ; c9 0d
b3c2: BEQ  $b3e2        ; f0 1e
b3c4: CMP  #$7f         ; c9 7f
b3c6: BEQ  $b3cc        ; f0 04
b3c8: CMP  #$08         ; c9 08
b3ca: BNE  $b3e2        ; d0 16
b3cc: LDX  $a7,X        ; a6 a7
b3ce: BEQ  $b3e1        ; f0 11
b3d0: DEC  $a7,X        ; c6 a7
b3d2: LDA  #$08         ; a9 08
b3d4: JSR  $a5a5        ; 20 a5 a5
b3d7: LDA  #$20         ; a9 20
b3d9: JSR  $a5a5        ; 20 a5 a5
b3dc: LDA  #$08         ; a9 08
b3de: JSR  $a5a5        ; 20 a5 a5
b3e1: RTS               ; 60 
b3e2: PHA               ; 48 
b3e3: JSR  $a5a5        ; 20 a5 a5
b3e6: LDA  $a7          ; a5 a7
b3e8: BNE  $b3fe        ; d0 14
b3ea: PLA               ; 68 
b3eb: PHA               ; 48 
b3ec: CMP  #$2f         ; c9 2f
b3ee: BNE  $b3fe        ; d0 0e
b3f0: PLA               ; 68 
b3f1: .byte $87   ;?
b3f2: .byte $93   ;?
b3f3: LDA  #$99         ; a9 99
b3f5: STA  $1239,X      ; 8d 39 12
b3f8: LDA  #$b4         ; a9 b4
b3fa: STA  $123a,X      ; 8d 3a 12
b3fd: RTS               ; 60 
b3fe: PLA               ; 68 
b3ff: CMP  #$0d         ; c9 0d
b401: BNE  $b418        ; d0 15
b403: .byte $87   ;?
b404: .byte $93   ;?
b405: LDA  #$0d         ; a9 0d
b407: PHA               ; 48 
b408: LDA  #$99         ; a9 99
b40a: STA  $1239,X      ; 8d 39 12
b40d: LDA  #$b4         ; a9 b4
b40f: STA  $123a,X      ; 8d 3a 12
b412: PLA               ; 68 
b413: JSR  $b431        ; 20 31 b4
b416: BRA  $b41b        ; 80 03
b418: JSR  $b423        ; 20 23 b4
b41b: BCC  $b420        ; 90 03
b41d: JMP  ($b3d2)      ; 4c d2 b3
b420: JMP  ($b3e1)      ; 4c e1 b3
b423: CMP  #$20         ; c9 20
b425: BCC  $b445        ; 90 1e
b427: CMP  #$61         ; c9 61
b429: BCC  $b431        ; 90 06
b42b: CMP  #$7b         ; c9 7b
b42d: BCS  $b431        ; b0 02
b42f: AND  #$5f         ; 29 5f
b431: LDX  $a7,X        ; a6 a7
b433: STA  ($a5),Y      ; 91 a5
b435: CPX  #$0e         ; e0 0e
b437: BNE  $b443        ; d0 0a
b439: CMP  #$0d         ; c9 0d
b43b: BEQ  $b443        ; f0 06
b43d: CMP  #$00         ; c9 00
b43f: BEQ  $b443        ; f0 02
b441: SEC               ; 38 
b442: RTS               ; 60 
b443: INC  $a7,X        ; e6 a7
b445: CLC               ; 18 
b446: RTS               ; 60 
b447: LDA  #$ff         ; a9 ff
b449: STA  $aa          ; 85 aa
b44b: LDA  ($56)        ; b2 56
b44d: LDA  $b2          ; a5 b2
b44f: ORA  ($a6)        ; 12 a6
b451: LDA  #$00         ; a9 00
b453: STA  $a7          ; 85 a7
b455: .byte $77   ;?
b456: .byte $9b   ;?
b457: RTS               ; 60 
b458: LDA  ($a5,X)      ; a1 a5
b45a: LDX  #$00         ; a2 00
b45c: CMP  $ec01,X      ; dd 01 ec
b45f: BNE  $b46e        ; d0 0d
b461: JSR  $b48d        ; 20 8d b4
b464: BCS  $b474        ; b0 0e
b466: LDA  ($2c)        ; b2 2c
b468: RTI               ; 40 
b469: LDA  ($ec)        ; b2 ec
b46b: EOR  ($80,X)      ; 41 80
b46d: TSB  ($bce8)      ; 0c e8 bc
b470: ORA  ($ec,X)      ; 01 ec
b472: BNE  $b45c        ; d0 e8
b474: LDA  ($2f)        ; b2 2f
b476: RTI               ; 40 
b477: LDA  ($ec)        ; b2 ec
b479: EOR  ($20,X)      ; 41 20
b47b: STZ  ($20b4)      ; 9c b4 20
b47e: .byte $47   ;?
b47f: LDY  $07,X        ; b4 07
b481: .byte $93   ;?
b482: LDA  #$be         ; a9 be
b484: STA  $1239,X      ; 8d 39 12
b487: LDA  #$b3         ; a9 b3
b489: STA  $123a,X      ; 8d 3a 12
b48c: RTS               ; 60 
b48d: INC  $a5,X        ; e6 a5
b48f: BNE  $b493        ; d0 02
b491: INC  $a6,X        ; e6 a6
b493: TXA               ; 8a 
b494: ASL               ; 0a 
b495: TAX               ; aa 
b496: JMP  ($ec08,X)    ; 7c 08 ec
b499: .byte $f7   ;?
b49a: .byte $9b   ;?
b49b: RTS               ; 60 
b49c: LDX  #$00         ; a2 00
b49e: LDA  ($40),Y      ; b1 40
b4a0: BEQ  $b4b9        ; f0 17
b4a2: CMP  #$0a         ; c9 0a
b4a4: BNE  $b4a9        ; d0 03
b4a6: INX               ; e8 
b4a7: BRA  $b4ac        ; 80 03
b4a9: JSR  $a5c2        ; 20 c2 a5
b4ac: LDA  ($40),Y      ; b1 40
b4ae: BEQ  $b4b6        ; f0 06
b4b0: JSR  $a5a5        ; 20 a5 a5
b4b3: INX               ; e8 
b4b4: BRA  $b4ac        ; 80 f6
b4b6: JSR  $a5c2        ; 20 c2 a5
b4b9: RTS               ; 60 
b4ba: .byte $27   ;?
b4bb: .byte $97   ;?
b4bc: .byte $07   ;?
b4bd: .byte $97   ;?
b4be: LDA  $a5          ; a5 a5
b4c0: STA  $40          ; 85 40
b4c2: LDA  $a6          ; a5 a6
b4c4: STA  $41          ; 85 41
b4c6: JSR  $b694        ; 20 94 b6
b4c9: LDY  #$00         ; a0 00
b4cb: LDX  #$ff         ; a2 ff
b4cd: LDA  ($40,X)      ; a1 40
b4cf: STA  $43          ; 85 43
b4d1: INX               ; e8 
b4d2: LDA  $ec14,X      ; bd 14 ec
b4d5: BEQ  $b4e1        ; f0 0a
b4d7: CMP  $43          ; c5 43
b4d9: BNE  $b4d1        ; d0 f6
b4db: TXA               ; 8a 
b4dc: ASL               ; 0a 
b4dd: TAX               ; aa 
b4de: JMP  ($ec1c,X)    ; 7c 1c ec
b4e1: SEC               ; 38 
b4e2: RTS               ; 60 
b4e3: JSR  $a5c2        ; 20 c2 a5
b4e6: JSR  $b583        ; 20 83 b5
b4e9: JSR  $b598        ; 20 98 b5
b4ec: JSR  $b68d        ; 20 8d b6
b4ef: LDA  $41          ; a5 41
b4f1: BEQ  $b4ec        ; f0 f9
b4f3: CMP  #$0d         ; c9 0d
b4f5: BEQ  $b4e3        ; f0 ec
b4f7: JSR  $a58e        ; 20 8e a5
b4fa: CLC               ; 18 
b4fb: RTS               ; 60 
b4fc: JSR  $a5c2        ; 20 c2 a5
b4ff: JSR  $b583        ; 20 83 b5
b502: JSR  $b5e6        ; 20 e6 b5
b505: STA  $45          ; 85 45
b507: JSR  $a5c9        ; 20 c9 a5
b50a: JSR  $b687        ; 20 87 b6
b50d: JSR  $b68d        ; 20 8d b6
b510: LDX  $41,X        ; a6 41
b512: BNE  $b4f7        ; d0 e3
b514: JSR  $b5e6        ; 20 e6 b5
b517: CMP  $45          ; c5 45
b519: BEQ  $b50d        ; f0 f2
b51b: PHA               ; 48 
b51c: JSR  $a5c2        ; 20 c2 a5
b51f: JSR  $b583        ; 20 83 b5
b522: PLA               ; 68 
b523: BRA  $b505        ; 80 e0
b525: JSR  $a5c2        ; 20 c2 a5
b528: JSR  $b583        ; 20 83 b5
b52b: JSR  $b5e6        ; 20 e6 b5
b52e: JSR  $a5c9        ; 20 c9 a5
b531: JSR  $b687        ; 20 87 b6
b534: JSR  $b68d        ; 20 8d b6
b537: LDA  $41          ; a5 41
b539: BEQ  $b534        ; f0 f9
b53b: CMP  #$0d         ; c9 0d
b53d: BEQ  $b555        ; f0 16
b53f: CMP  #$20         ; c9 20
b541: BEQ  $b4f7        ; f0 b4
b543: JSR  $b5ca        ; 20 ca b5
b546: JSR  $b617        ; 20 17 b6
b549: JSR  $a5c2        ; 20 c2 a5
b54c: JSR  $b583        ; 20 83 b5
b54f: JSR  $b5e6        ; 20 e6 b5
b552: JSR  $a5c9        ; 20 c9 a5
b555: INC  $46,X        ; e6 46
b557: BNE  $b55b        ; d0 02
b559: INC  $47,X        ; e6 47
b55b: BRA  $b525        ; 80 c8
b55d: .byte $a7   ;?
b55e: .byte $97   ;?
b55f: BRA  $b565        ; 80 04
b561: .byte $87   ;?
b562: .byte $97   ;?
b563: BRA  $b565        ; 80 00
b565: JSR  $b67a        ; 20 7a b6
b568: JMP  ($b4cb)      ; 4c cb b4
b56b: JSR  $b583        ; 20 83 b5
b56e: .byte $c7   ;?
b56f: .byte $97   ;?
b570: JSR  $b598        ; 20 98 b5
b573: .byte $47   ;?
b574: .byte $97   ;?
b575: JMP  ($b4f7)      ; 4c f7 b4
b578: LDX  #$00         ; a2 00
b57a: JSR  $b64b        ; 20 4b b6
b57d: JSR  $b611        ; 20 11 b6
b580: JMP  ($b56b)      ; 4c 6b b5
b583: LDA  $47          ; a5 47
b585: JSR  $a5c9        ; 20 c9 a5
b588: LDA  $46          ; a5 46
b58a: JSR  $a5c9        ; 20 c9 a5
b58d: LDA  #$3a         ; a9 3a
b58f: JSR  $a5a5        ; 20 a5 a5
b592: JSR  $b687        ; 20 87 b6
b595: LDX  #$00         ; a2 00
b597: RTS               ; 60 
b598: LDX  #$00         ; a2 00
b59a: JSR  $b5e6        ; 20 e6 b5
b59d: JSR  $a5c9        ; 20 c9 a5
b5a0: JSR  $b687        ; 20 87 b6
b5a3: .byte $af   ;?
b5a4: .byte $97   ;?
b5a5: ASL  $cf,X        ; 16 cf
b5a7: .byte $97   ;?
b5a8: .byte $13   ;?
b5a9: INX               ; e8 
b5aa: CPX  #$08         ; e0 08
b5ac: BNE  $b59a        ; d0 ec
b5ae: CLC               ; 18 
b5af: LDA  $46          ; a5 46
b5b1: ADC  #$08         ; 69 08
b5b3: STA  $46          ; 85 46
b5b5: LDA  $47          ; a5 47
b5b7: ADC  #$00         ; 69 00
b5b9: STA  $47          ; 85 47
b5bb: RTS               ; 60 
b5bc: CLC               ; 18 
b5bd: LDA  $46          ; a5 46
b5bf: ADC  #$01         ; 69 01
b5c1: STA  $46          ; 85 46
b5c3: LDA  $47          ; a5 47
b5c5: ADC  #$00         ; 69 00
b5c7: STA  $47          ; 85 47
b5c9: RTS               ; 60 
b5ca: JSR  $a5a5        ; 20 a5 a5
b5cd: JSR  $a5e3        ; 20 e3 a5
b5d0: ASL               ; 0a 
b5d1: ASL               ; 0a 
b5d2: ASL               ; 0a 
b5d3: ASL               ; 0a 
b5d4: STA  $42          ; 85 42
b5d6: JSR  $b68d        ; 20 8d b6
b5d9: LDA  $41          ; a5 41
b5db: BEQ  $b5d6        ; f0 f9
b5dd: JSR  $a5a5        ; 20 a5 a5
b5e0: JSR  $a5e3        ; 20 e3 a5
b5e3: ORA  $42          ; 05 42
b5e5: RTS               ; 60 
b5e6: .byte $0f   ;?
b5e7: .byte $97   ;?
b5e8: .byte $0b   ;?
b5e9: LDA  $47          ; a5 47
b5eb: STA  $41          ; 85 41
b5ed: LDA  $46          ; a5 46
b5ef: STA  $40          ; 85 40
b5f1: JMP  ($eae5)      ; 4c e5 ea
b5f4: .byte $2f   ;?
b5f5: .byte $97   ;?
b5f6: ASL  $47a5        ; 0e a5 47
b5f9: STA  $41          ; 85 41
b5fb: LDA  $46          ; a5 46
b5fd: STA  $40          ; 85 40
b5ff: LDX  #$00         ; a2 00
b601: JSR  $ac1d        ; 20 1d ac
b604: RTS               ; 60 
b605: .byte $17   ;?
b606: .byte $97   ;?
b607: .byte $9f   ;?
b608: .byte $97   ;?
b609: TSB  $b1          ; 04 b1
b60b: LSR  $80,X        ; 46 80
b60d: .byte $02   ;?
b60e: STA  ($46),Y      ; 91 46
b610: RTS               ; 60 
b611: JSR  $b617        ; 20 17 b6
b614: JMP  ($b67a)      ; 4c 7a b6
b617: .byte $0f   ;?
b618: .byte $97   ;?
b619: ASL  $48,X        ; 16 48
b61b: LDA  $47          ; a5 47
b61d: STA  $43          ; 85 43
b61f: LDA  $46          ; a5 46
b621: STA  $42          ; 85 42
b623: PLA               ; 68 
b624: JSR  $eb22        ; 20 22 eb
b627: LDA  $42          ; a5 42
b629: STA  $46          ; 85 46
b62b: LDA  $43          ; a5 43
b62d: STA  $47          ; 85 47
b62f: RTS               ; 60 
b630: .byte $2f   ;?
b631: .byte $97   ;?
b632: RTI               ; 40 
b633: PHA               ; 48 
b634: LDA  $47          ; a5 47
b636: STA  $43          ; 85 43
b638: LDA  $46          ; a5 46
b63a: STA  $42          ; 85 42
b63c: PLA               ; 68 
b63d: LDX  #$00         ; a2 00
b63f: JSR  $ac5b        ; 20 5b ac
b642: LDA  $42          ; a5 42
b644: STA  $46          ; 85 46
b646: LDA  $43          ; a5 43
b648: STA  $47          ; 85 47
b64a: RTS               ; 60 
b64b: JSR  $b67a        ; 20 7a b6
b64e: LDA  ($40,X)      ; a1 40
b650: CMP  #$0d         ; c9 0d
b652: BEQ  $b66e        ; f0 1a
b654: JSR  $a5e3        ; 20 e3 a5
b657: PHA               ; 48 
b658: JSR  $b67a        ; 20 7a b6
b65b: LDA  ($40,X)      ; a1 40
b65d: CMP  #$0d         ; c9 0d
b65f: BEQ  $b671        ; f0 10
b661: JSR  $a5e3        ; 20 e3 a5
b664: STA  $42          ; 85 42
b666: PLA               ; 68 
b667: ASL               ; 0a 
b668: ASL               ; 0a 
b669: ASL               ; 0a 
b66a: ASL               ; 0a 
b66b: ORA  $42          ; 05 42
b66d: RTS               ; 60 
b66e: JMP  ($b4f7)      ; 4c f7 b4
b671: PLA               ; 68 
b672: RTS               ; 60 
b673: PHA               ; 48 
b674: .byte $97   ;?
b675: .byte $97   ;?
b676: PLA               ; 68 
b677: JMP  ($b607)      ; 4c 07 b6
b67a: INC  $a5,X        ; e6 a5
b67c: BNE  $b680        ; d0 02
b67e: INC  $a6,X        ; e6 a6
b680: INC  $40,X        ; e6 40
b682: BNE  $b686        ; d0 02
b684: INC  $41,X        ; e6 41
b686: RTS               ; 60 
b687: LDA  #$20         ; a9 20
b689: JSR  $a5a5        ; 20 a5 a5
b68c: RTS               ; 60 
b68d: JSR  $a591        ; 20 91 a5
b690: JSR  $a594        ; 20 94 a5
b693: RTS               ; 60 
b694: LDA  ($00)        ; b2 00
b696: .byte $43   ;?
b697: LDX  #$00         ; a2 00
b699: JSR  $b6b3        ; 20 b3 b6
b69c: STA  $47          ; 85 47
b69e: JSR  $b6b3        ; 20 b3 b6
b6a1: STA  $46          ; 85 46
b6a3: STX  $42,X        ; 86 42
b6a5: CLC               ; 18 
b6a6: LDA  $40          ; a5 40
b6a8: ADC  $42          ; 65 42
b6aa: STA  $40          ; 85 40
b6ac: LDA  $41          ; a5 41
b6ae: ADC  #$00         ; 69 00
b6b0: STA  $41          ; 85 41
b6b2: RTS               ; 60 
b6b3: LDA  ($40),Y      ; b1 40
b6b5: JSR  $a5e3        ; 20 e3 a5
b6b8: ASL               ; 0a 
b6b9: ASL               ; 0a 
b6ba: ASL               ; 0a 
b6bb: ASL               ; 0a 
b6bc: INX               ; e8 
b6bd: INC  $a5,X        ; e6 a5
b6bf: BNE  $b6c3        ; d0 02
b6c1: INC  $a6,X        ; e6 a6
b6c3: STA  $42          ; 85 42
b6c5: LDA  ($40),Y      ; b1 40
b6c7: JSR  $a5e3        ; 20 e3 a5
b6ca: ORA  $42          ; 05 42
b6cc: INX               ; e8 
b6cd: INC  $a5,X        ; e6 a5
b6cf: BNE  $b6d3        ; d0 02
b6d1: INC  $a6,X        ; e6 a6
b6d3: RTS               ; 60 
b6d4: LDA  ($74)        ; b2 74
b6d6: RTI               ; 40 
b6d7: LDA  ($ec)        ; b2 ec
b6d9: EOR  ($20,X)      ; 41 20
b6db: STZ  ($0cb4)      ; 9c b4 0c
b6de: TRB  $d5          ; 54 d5
b6e0: .byte $43   ;?
b6e1: BVC  $b733        ; 50 50
b6e3: LSR  $3a,X        ; 56 3a
b6e5: BRK               ; 00 
b6e6: LDX  #$00         ; a2 00
b6e8: LDA  $148f,X      ; bd 8f 14
b6eb: JSR  $a5c9        ; 20 c9 a5
b6ee: INX               ; e8 
b6ef: CPX  #$03         ; e0 03
b6f1: BCS  $b6fa        ; b0 07
b6f3: TSB  ($d554)      ; 0c 54 d5
b6f6: ROL  $8000        ; 2e 00 80
b6f9: INC  $9b3f        ; ee 3f 9b
b6fc: .byte $33   ;?
b6fd: LDA  ($20)        ; b2 20
b6ff: RTI               ; 40 
b700: LDA  ($00)        ; b2 00
b702: EOR  ($20,X)      ; 41 20
b704: AND  ($b7)        ; 32 b7
b706: BCS  $b71e        ; b0 16
b708: LDA  ($74)        ; b2 74
b70a: RTI               ; 40 
b70b: LDA  ($01)        ; b2 01
b70d: EOR  ($20,X)      ; 41 20
b70f: AND  ($b7)        ; 32 b7
b711: BCS  $b71e        ; b0 0b
b713: LDA  ($83)        ; b2 83
b715: RTI               ; 40 
b716: LDA  ($01)        ; b2 01
b718: EOR  ($20,X)      ; 41 20
b71a: AND  ($b7)        ; 32 b7
b71c: BCC  $b730        ; 90 12
b71e: JSR  $a5c2        ; 20 c2 a5
b721: LDX  #$00         ; a2 00
b723: JSR  $eb9e        ; 20 9e eb
b726: CMP  #$00         ; c9 00
b728: BEQ  $b730        ; f0 06
b72a: JSR  $a5a5        ; 20 a5 a5
b72d: INX               ; e8 
b72e: BRA  $b723        ; 80 f3
b730: CLC               ; 18 
b731: RTS               ; 60 
b732: JSR  $eb95        ; 20 95 eb
b735: STA  $42          ; 85 42
b737: INX               ; e8 
b738: JSR  $eb9e        ; 20 9e eb
b73b: STA  $41          ; 85 41
b73d: CMP  $42          ; c5 42
b73f: BNE  $b74b        ; d0 0a
b741: CMP  #$00         ; c9 00
b743: BEQ  $b749        ; f0 04
b745: CMP  #$ff         ; c9 ff
b747: BNE  $b74b        ; d0 02
b749: CLC               ; 18 
b74a: RTS               ; 60 
b74b: LDA  $42          ; a5 42
b74d: STA  $40          ; 85 40
b74f: SEC               ; 38 
b750: RTS               ; 60 
b751: CLC               ; 18 
b752: RTS               ; 60 
b753: .byte $d7   ;?
b754: .byte $97   ;?
b755: .byte $c7   ;?
b756: TYA               ; 98 
b757: JSR  $af41        ; 20 41 af
b75a: PHP               ; 08 
b75b: LDA  ($5b)        ; b2 5b
b75d: RTI               ; 40 
b75e: LDA  ($ec)        ; b2 ec
b760: EOR  ($b0,X)      ; 41 b0
b762: ASL  $b2,X        ; 06 b2
b764: AND  $40          ; 35 40
b766: LDA  ($ec)        ; b2 ec
b768: EOR  ($20,X)      ; 41 20
b76a: STZ  ($20b4)      ; 9c b4 20
b76d: .byte $c2   ;?
b76e: LDA  $57          ; a5 57
b770: .byte $97   ;?
b771: PLP               ; 28 
b772: RTS               ; 60 
b773: LDA  ($00)        ; b2 00
b775: LSR  $b2,X        ; 46 b2
b777: .byte $0f   ;?
b778: .byte $47   ;?
b779: LDA  ($a5,X)      ; a1 a5
b77b: CMP  #$56         ; c9 56
b77d: BEQ  $b79b        ; f0 1c
b77f: LDA  ($46,X)      ; a1 46
b781: JSR  $a5a5        ; 20 a5 a5
b784: INC  $46,X        ; e6 46
b786: BNE  $b78a        ; d0 02
b788: INC  $47,X        ; e6 47
b78a: LDA  $47          ; a5 47
b78c: CMP  #$12         ; c9 12
b78e: BNE  $b794        ; d0 04
b790: LDA  $46          ; a5 46
b792: CMP  #$38         ; c9 38
b794: BCC  $b77f        ; 90 e9
b796: PLA               ; 68 
b797: PLA               ; 68 
b798: JMP  ($b47d)      ; 4c 7d b4
b79b: INC  $a5,X        ; e6 a5
b79d: BNE  $b7a1        ; d0 02
b79f: INC  $a6,X        ; e6 a6
b7a1: JSR  $a5c2        ; 20 c2 a5
b7a4: JSR  $b583        ; 20 83 b5
b7a7: LDA  ($46,X)      ; a1 46
b7a9: JSR  $a5c9        ; 20 c9 a5
b7ac: INC  $46,X        ; e6 46
b7ae: BNE  $b7b2        ; d0 02
b7b0: INC  $47,X        ; e6 47
b7b2: LDA  $47          ; a5 47
b7b4: CMP  #$12         ; c9 12
b7b6: BNE  $b7bc        ; d0 04
b7b8: LDA  $46          ; a5 46
b7ba: CMP  #$38         ; c9 38
b7bc: BCC  $b7a1        ; 90 e3
b7be: CLC               ; 18 
b7bf: RTS               ; 60 
b7c0: JSR  $13ed        ; 20 ed 13
b7c3: JSR  $b862        ; 20 62 b8
b7c6: JSR  $b8a7        ; 20 a7 b8
b7c9: LDA  ($7d)        ; b2 7d
b7cb: RTI               ; 40 
b7cc: LDA  ($ec)        ; b2 ec
b7ce: EOR  ($20,X)      ; 41 20
b7d0: CPX  $a5          ; f4 a5
b7d2: STA  $1002,X      ; 8d 02 10
b7d5: JSR  $a84a        ; 20 4a a8
b7d8: JSR  $a5f4        ; 20 f4 a5
b7db: STA  $1001,X      ; 8d 01 10
b7de: JSR  $a84a        ; 20 4a a8
b7e1: JSR  $a5f4        ; 20 f4 a5
b7e4: STA  $1004,X      ; 8d 04 10
b7e7: JSR  $a84a        ; 20 4a a8
b7ea: JSR  $a5f4        ; 20 f4 a5
b7ed: STA  $1003,X      ; 8d 03 10
b7f0: LDA  #$0f         ; a9 0f
b7f2: STA  $1006,X      ; 8d 06 10
b7f5: LDA  $66          ; a5 66
b7f7: STA  $126a,X      ; 8d 6a 12
b7fa: STA  $126c,X      ; 8d 6c 12
b7fd: LDA  $67          ; a5 67
b7ff: STA  $126b,X      ; 8d 6b 12
b802: STA  $126d,X      ; 8d 6d 12
b805: LDA  $6e          ; a5 6e
b807: STA  $126e,X      ; 8d 6e 12
b80a: STA  $1270,X      ; 8d 70 12
b80d: LDA  $6f          ; a5 6f
b80f: STA  $126f,X      ; 8d 6f 12
b812: STA  $1271,X      ; 8d 71 12
b815: LDA  $76          ; a5 76
b817: STA  $1276,X      ; 8d 76 12
b81a: STA  $1278,X      ; 8d 78 12
b81d: LDA  $77          ; a5 77
b81f: STA  $1277,X      ; 8d 77 12
b822: STA  $1279,X      ; 8d 79 12
b825: LDA  $7e          ; a5 7e
b827: STA  $127a,X      ; 8d 7a 12
b82a: LDA  $7f          ; a5 7f
b82c: STA  $127b,X      ; 8d 7b 12
b82f: LDA  $86          ; a5 86
b831: STA  $1272,X      ; 8d 72 12
b834: STA  $1274,X      ; 8d 74 12
b837: LDA  $87          ; a5 87
b839: STA  $1273,X      ; 8d 73 12
b83c: STA  $1275,X      ; 8d 75 12
b83f: .byte $97   ;?
b840: ORA  $0f          ; 05 0f
b842: .byte $4f   ;?
b843: .byte $07   ;?
b844: .byte $67   ;?
b845: TSB  $b2          ; 04 b2
b847: BRK               ; 00 
b848: DEA               ; 3a 
b849: BRA  $b850        ; 80 05
b84b: .byte $e7   ;?
b84c: TSB  $b2          ; 04 b2
b84e: CPY  #$3a         ; c0 3a
b850: .byte $2f   ;?
b851: EOR  $3f05,X      ; 4d 05 3f
b854: EOR  $7702,X      ; 4d 02 77
b857: ASL  $d2,X        ; 06 d2
b859: ORA  ($c8,X)      ; 01 c8
b85b: ORA  ($a9),Y      ; 11 a9
b85d: ORA  ($8d,X)      ; 01 8d
b85f: BRK               ; 00 
b860: BPL  $b8c2        ; 10 60
b862: LDX  #$46         ; a2 46
b864: LDA  $f7d6,X      ; bd d6 f7
b867: STA  $4c          ; 95 4c
b869: DEX               ; ca 
b86a: BPL  $b864        ; 10 f8
b86c: LDA  #$1d         ; a9 1d
b86e: STA  $40          ; 85 40
b870: LDA  #$f8         ; a9 f8
b872: STA  $41          ; 85 41
b874: LDA  #$00         ; a9 00
b876: STA  $42          ; 85 42
b878: STA  $43          ; 85 43
b87a: LDA  #$00         ; a9 00
b87c: STA  $46          ; 85 46
b87e: LDA  #$10         ; a9 10
b880: STA  $47          ; 85 47
b882: LDA  ($40,X)      ; a1 40
b884: STA  ($46,X)      ; 81 46
b886: LDA  $43          ; a5 43
b888: CMP  #$02         ; c9 02
b88a: BNE  $b890        ; d0 04
b88c: LDA  $42          ; a5 42
b88e: CMP  #$37         ; c9 37
b890: BEQ  $b8a6        ; f0 14
b892: INC  $46,X        ; e6 46
b894: BNE  $b898        ; d0 02
b896: INC  $47,X        ; e6 47
b898: INC  $42,X        ; e6 42
b89a: BNE  $b89e        ; d0 02
b89c: INC  $43,X        ; e6 43
b89e: INC  $40,X        ; e6 40
b8a0: BNE  $b8a4        ; d0 02
b8a2: INC  $41,X        ; e6 41
b8a4: BRA  $b882        ; 80 dc
b8a6: RTS               ; 60 
b8a7: .byte $3f   ;?
b8a8: .byte $9b   ;?
b8a9: ROL               ; 2a 
b8aa: LDA  ($00)        ; b2 00
b8ac: EOR  #$b2         ; 49 b2
b8ae: ORA  ($40),Y      ; 11 40
b8b0: LDA  ($00)        ; b2 00
b8b2: EOR  ($20,X)      ; 41 20
b8b4: STA  $eb          ; 95 eb
b8b6: CMP  #$81         ; c9 81
b8b8: BEQ  $b8d6        ; f0 1c
b8ba: LDA  ($6e)        ; b2 6e
b8bc: RTI               ; 40 
b8bd: LDA  ($01)        ; b2 01
b8bf: EOR  ($20,X)      ; 41 20
b8c1: STA  $eb          ; 95 eb
b8c3: CMP  #$81         ; c9 81
b8c5: BEQ  $b8d6        ; f0 0f
b8c7: LDA  ($7d)        ; b2 7d
b8c9: RTI               ; 40 
b8ca: LDA  ($01)        ; b2 01
b8cc: EOR  ($20,X)      ; 41 20
b8ce: STA  $eb          ; 95 eb
b8d0: CMP  #$81         ; c9 81
b8d2: BEQ  $b8d6        ; f0 02
b8d4: BRA  $b928        ; 80 52
b8d6: INX               ; e8 
b8d7: JSR  $eb9e        ; 20 9e eb
b8da: PHA               ; 48 
b8db: INX               ; e8 
b8dc: JSR  $eb9e        ; 20 9e eb
b8df: STA  $41          ; 85 41
b8e1: PLA               ; 68 
b8e2: STA  $40          ; 85 40
b8e4: LDX  #$00         ; a2 00
b8e6: .byte $57   ;?
b8e7: TYA               ; 98 
b8e8: JSR  $eba7        ; 20 a7 eb
b8eb: STA  $4c          ; 95 4c
b8ed: INX               ; e8 
b8ee: CPX  #$47         ; e0 47
b8f0: BCC  $b8e8        ; 90 f6
b8f2: LDA  #$00         ; a9 00
b8f4: STA  $42          ; 85 42
b8f6: STA  $43          ; 85 43
b8f8: LDA  #$00         ; a9 00
b8fa: STA  $46          ; 85 46
b8fc: LDA  #$10         ; a9 10
b8fe: STA  $47          ; 85 47
b900: JSR  $eba7        ; 20 a7 eb
b903: STA  ($46,X)      ; 81 46
b905: LDA  $43          ; a5 43
b907: CMP  #$02         ; c9 02
b909: BNE  $b90f        ; d0 04
b90b: LDA  $42          ; a5 42
b90d: CMP  #$37         ; c9 37
b90f: BEQ  $b91f        ; f0 0e
b911: INC  $46,X        ; e6 46
b913: BNE  $b917        ; d0 02
b915: INC  $47,X        ; e6 47
b917: INC  $42,X        ; e6 42
b919: BNE  $b91d        ; d0 02
b91b: INC  $43,X        ; e6 43
b91d: BRA  $b900        ; 80 e1
b91f: .byte $ff   ;?
b920: TYA               ; 98 
b921: .byte $03   ;?
b922: JSR  $ae56        ; 20 56 ae
b925: LDA  ($01)        ; b2 01
b927: EOR  #$ad         ; 49 ad
b929: ASL               ; 0a 
b92a: TSB  $29          ; 04 29
b92c: BVS  $b933        ; 70 05
b92e: EOR  #$8d         ; 49 8d
b930: ASL               ; 0a 
b931: TSB  $60          ; 04 60
b933: JSR  $13f3        ; 20 f3 13
b936: .byte $e2   ;?
b937: ASL               ; 0a 
b938: TSB  $80          ; 04 80
b93a: .byte $0b   ;?
b93b: .byte $bf   ;?
b93c: .byte $9b   ;?
b93d: ORA  $0aad,X      ; 0d ad 0a
b940: TSB  $29          ; 04 29
b942: BVS  $b8d1        ; 70 8d
b944: ASL               ; 0a 
b945: TSB  $c2          ; 04 c2
b947: BRA  $b953        ; 80 0a
b949: TSB  $60          ; 04 60
b94b: .byte $17   ;?
b94c: TYA               ; 98 
b94d: LDA  ($11)        ; b2 11
b94f: RTI               ; 40 
b950: LDA  ($00)        ; b2 00
b952: EOR  ($20,X)      ; 41 20
b954: STA  $eb          ; 95 eb
b956: CMP  #$80         ; c9 80
b958: BEQ  $b980        ; f0 26
b95a: CMP  #$01         ; c9 01
b95c: BEQ  $b980        ; f0 22
b95e: LDA  ($6e)        ; b2 6e
b960: RTI               ; 40 
b961: LDA  ($01)        ; b2 01
b963: EOR  ($20,X)      ; 41 20
b965: STA  $eb          ; 95 eb
b967: CMP  #$80         ; c9 80
b969: BEQ  $b980        ; f0 15
b96b: CMP  #$01         ; c9 01
b96d: BEQ  $b980        ; f0 11
b96f: LDA  ($11)        ; b2 11
b971: RTI               ; 40 
b972: LDA  ($00)        ; b2 00
b974: EOR  ($20,X)      ; 41 20
b976: STA  $eb          ; 95 eb
b978: CMP  #$80         ; c9 80
b97a: BEQ  $b980        ; f0 04
b97c: CMP  #$01         ; c9 01
b97e: BNE  $b93e        ; d0 be
b980: LDA  $40          ; a5 40
b982: STA  $42          ; 85 42
b984: LDA  $41          ; a5 41
b986: STA  $43          ; 85 43
b988: SBC  ($0a)        ; f2 0a
b98a: TSB  $40          ; 04 40
b98c: .byte $3b   ;?
b98d: .byte $7f   ;?
b98e: TYA               ; 98 
b98f: .byte $13   ;?
b990: LDA  $040a,X      ; ad 0a 04
b993: AND  #$0f         ; 29 0f
b995: CMP  #$01         ; c9 01
b997: BEQ  $b99e        ; f0 05
b999: JSR  $b862        ; 20 62 b8
b99c: BRA  $b9b7        ; 80 19
b99e: JSR  $b8a7        ; 20 a7 b8
b9a1: BRA  $b9b7        ; 80 14
b9a3: JSR  $aea7        ; 20 a7 ae
b9a6: AND  #$f0         ; 29 f0
b9a8: STA  $44          ; 85 44
b9aa: LDA  $040a,X      ; ad 0a 04
b9ad: AND  #$0f         ; 29 0f
b9af: ORA  $44          ; 05 44
b9b1: JSR  $aea1        ; 20 a1 ae
b9b4: JSR  $b7c0        ; 20 c0 b7
b9b7: .byte $17   ;?
b9b8: LDX  #$27         ; a2 27
b9ba: LDX  #$f2         ; a2 f2
b9bc: BRK               ; 00 
b9bd: BPL  $b93f        ; 10 80
b9bf: .byte $03   ;?
b9c0: JSR  $ba3f        ; 20 3f ba
b9c3: .byte $c2   ;?
b9c4: BRA  $b9d0        ; 80 0a
b9c6: TSB  $60          ; 04 60
b9c8: .byte $0f   ;?
b9c9: TYA               ; 98 
b9ca: SED               ; f8 
b9cb: LDA  $040a,X      ; ad 0a 04
b9ce: AND  #$0f         ; 29 0f
b9d0: CMP  #$01         ; c9 01
b9d2: BEQ  $b9d7        ; f0 03
b9d4: JMP  ($b93e)      ; 4c 3e b9
b9d7: LDA  $42          ; a5 42
b9d9: PHA               ; 48 
b9da: LDA  $43          ; a5 43
b9dc: PHA               ; 48 
b9dd: JSR  $aea1        ; 20 a1 ae
b9e0: INX               ; e8 
b9e1: JSR  $aea7        ; 20 a7 ae
b9e4: PHA               ; 48 
b9e5: INX               ; e8 
b9e6: JSR  $aea7        ; 20 a7 ae
b9e9: STA  $43          ; 85 43
b9eb: PLA               ; 68 
b9ec: STA  $42          ; 85 42
b9ee: LDX  #$00         ; a2 00
b9f0: LDA  $4c          ; b5 4c
b9f2: JSR  $acbf        ; 20 bf ac
b9f5: INX               ; e8 
b9f6: CPX  #$47         ; e0 47
b9f8: BCC  $b9f0        ; 90 f6
b9fa: LDA  #$00         ; a9 00
b9fc: STA  $40          ; 85 40
b9fe: STA  $41          ; 85 41
ba00: LDA  #$00         ; a9 00
ba02: STA  $46          ; 85 46
ba04: LDA  #$10         ; a9 10
ba06: STA  $47          ; 85 47
ba08: LDA  ($46,X)      ; a1 46
ba0a: JSR  $acbf        ; 20 bf ac
ba0d: LDA  $41          ; a5 41
ba0f: CMP  #$02         ; c9 02
ba11: BNE  $ba17        ; d0 04
ba13: LDA  $40          ; a5 40
ba15: CMP  #$37         ; c9 37
ba17: BEQ  $ba27        ; f0 0e
ba19: INC  $46,X        ; e6 46
ba1b: BNE  $ba1f        ; d0 02
ba1d: INC  $47,X        ; e6 47
ba1f: INC  $40,X        ; e6 40
ba21: BNE  $ba25        ; d0 02
ba23: INC  $41,X        ; e6 41
ba25: BRA  $ba08        ; 80 e1
ba27: .byte $0f   ;?
ba28: TXS               ; 9a 
ba29: .byte $03   ;?
ba2a: JSR  $acf0        ; 20 f0 ac
ba2d: PLA               ; 68 
ba2e: STA  $43          ; 85 43
ba30: PLA               ; 68 
ba31: STA  $42          ; 85 42
ba33: LDA  #$81         ; a9 81
ba35: LDX  #$00         ; a2 00
ba37: JSR  $aea1        ; 20 a1 ae
ba3a: .byte $c2   ;?
ba3b: BRA  $ba47        ; 80 0a
ba3d: TSB  $60          ; 04 60
ba3f: .byte $e2   ;?
ba40: INY               ; c8 
ba41: ORA  ($01),Y      ; 11 01
ba43: .byte $17   ;?
ba44: JSR  $ba7c        ; 20 7c ba
ba47: JSR  $db0c        ; 20 0c db
ba4a: JSR  $db53        ; 20 53 db
ba4d: JSR  $db9c        ; 20 9c db
ba50: JSR  $dac5        ; 20 c5 da
ba53: LDA  #$ff         ; a9 ff
ba55: STA  $aa          ; 85 aa
ba57: .byte $c2   ;?
ba58: ORA  ($c8,X)      ; 01 c8
ba5a: ORA  ($3f),Y      ; 11 3f
ba5c: STY  $14,X        ; 94 14
ba5e: LDA  $0f18,X      ; ad 18 0f
ba61: AND  #$07         ; 29 07
ba63: BNE  $ba69        ; d0 04
ba65: CMP  ($20)        ; d2 20
ba67: BIT  ($f20f)      ; 2c 0f f2
ba6a: ORA  $200f,X      ; 1d 0f 20
ba6d: TSB  $d2          ; 04 d2
ba6f: RTI               ; 40 
ba70: BIT  ($200f)      ; 2c 0f 20
ba73: LDX  $e0,X        ; a6 e0
ba75: JSR  $e199        ; 20 99 e1
ba78: JSR  $dcf5        ; 20 f5 dc
ba7b: RTS               ; 60 
ba7c: JSR  $140b        ; 20 0b 14
ba7f: SBC  ($c8)        ; f2 c8
ba81: ORA  ($02),Y      ; 11 02
ba83: ORA  #$f2         ; 09 f2
ba85: INY               ; c8 
ba86: ORA  ($08),Y      ; 11 08
ba88: TSB  $a9          ; 04 a9
ba8a: ASL  $80,X        ; 06 80
ba8c: .byte $02   ;?
ba8d: LDA  #$00         ; a9 00
ba8f: STA  $1268,X      ; 8d 68 12
ba92: SBC  ($c8)        ; f2 c8
ba94: ORA  ($04),Y      ; 11 04
ba96: TSB  $a9          ; 04 a9
ba98: .byte $03   ;?
ba99: BRA  $ba9d        ; 80 02
ba9b: LDA  #$00         ; a9 00
ba9d: STA  $1269,X      ; 8d 69 12
baa0: RTS               ; 60 
baa1: LDA  $11c9,X      ; ad c9 11
baa4: AND  #$07         ; 29 07
baa6: JMP  ($a791)      ; 4c 91 a7
baa9: JSR  $13bd        ; 20 bd 13
baac: LDX  #$01         ; a2 01
baae: JSR  $a51f        ; 20 1f a5
bab1: .byte $c2   ;?
bab2: TSB  $78          ; 04 78
bab4: TSB  $20          ; 04 20
bab6: .byte $7b   ;?
bab7: SBC  $78          ; e5 78
bab9: LDA  $04          ; a5 04
babb: STA  $40          ; 85 40
babd: LDA  $05          ; a5 05
babf: STA  $41          ; 85 41
bac1: LDA  $06          ; a5 06
bac3: STA  $42          ; 85 42
bac5: LDA  $00          ; a5 00
bac7: STA  $43          ; 85 43
bac9: LDA  $01          ; a5 01
bacb: STA  $44          ; 85 44
bacd: LDA  $02          ; a5 02
bacf: STA  $45          ; 85 45
bad1: .byte $27   ;?
bad2: TSB  $0f          ; 04 0f
bad4: TYA               ; 98 
bad5: ASL  $c7,X        ; 06 c7
bad7: TSB  $c7          ; 04 c7
bad9: BRK               ; 00 
bada: BRA  $bade        ; 80 02
badc: .byte $47   ;?
badd: TSB  $d7          ; 04 d7
badf: TSB  $4f          ; 04 4f
bae1: LDA  ($04,X)      ; a1 04
bae3: .byte $d7   ;?
bae4: BRK               ; 00 
bae5: BRA  $baed        ; 80 06
bae7: CMP  ($01)        ; d2 01
bae9: CPY  #$04         ; c0 04
baeb: .byte $57   ;?
baec: BRK               ; 00 
baed: .byte $8f   ;?
baee: .byte $4f   ;?
baef: ASL  $e7,X        ; 06 e7
baf1: TSB  $e7          ; 04 e7
baf3: BRK               ; 00 
baf4: BRA  $baf8        ; 80 02
baf6: .byte $67   ;?
baf7: TSB  $87          ; 04 87
baf9: ORA  $07          ; 05 07
bafb: ORA  ($97,X)      ; 01 97
bafd: ORA  $17          ; 05 17
baff: ORA  ($a7,X)      ; 01 a7
bb01: ORA  $4f          ; 05 4f
bb03: LDA  ($04,X)      ; a1 04
bb05: .byte $a7   ;?
bb06: ORA  ($80,X)      ; 01 80
bb08: .byte $02   ;?
bb09: .byte $27   ;?
bb0a: ORA  ($b7,X)      ; 01 b7
bb0c: ORA  $37          ; 05 37
bb0e: ORA  ($c7,X)      ; 01 c7
bb10: ORA  $c7          ; 05 c7
bb12: ORA  ($d7,X)      ; 01 d7
bb14: ORA  $57          ; 05 57
bb16: ORA  ($e7,X)      ; 01 e7
bb18: ORA  $e7          ; 05 e7
bb1a: ORA  ($77,X)      ; 01 77
bb1c: ORA  $a9          ; 05 a9
bb1e: BRK               ; 00 
bb1f: STA  $06          ; 85 06
bb21: LDA  #$00         ; a9 00
bb23: .byte $4f   ;?
bb24: LDA  ($00,X)      ; a1 00
bb26: STA  $02          ; 85 02
bb28: LDA  #$00         ; a9 00
bb2a: .byte $4f   ;?
bb2b: LDA  ($02,X)      ; a1 02
bb2d: LDA  #$0f         ; a9 0f
bb2f: STA  $0f6f,X      ; 8d 6f 0f
bb32: LDA  #$04         ; a9 04
bb34: STA  $0440,X      ; 8d 40 04
bb37: LDA  #$00         ; a9 00
bb39: STA  $0441,X      ; 8d 41 04
bb3c: LDA  #$e4         ; a9 e4
bb3e: STA  $09          ; 85 09
bb40: NOP               ; ea 
bb41: NOP               ; ea 
bb42: NOP               ; ea 
bb43: NOP               ; ea 
bb44: NOP               ; ea 
bb45: NOP               ; ea 
bb46: NOP               ; ea 
bb47: NOP               ; ea 
bb48: NOP               ; ea 
bb49: NOP               ; ea 
bb4a: NOP               ; ea 
bb4b: NOP               ; ea 
bb4c: NOP               ; ea 
bb4d: NOP               ; ea 
bb4e: NOP               ; ea 
bb4f: NOP               ; ea 
bb50: NOP               ; ea 
bb51: .byte $0f   ;?
bb52: ORA  #$fd         ; 09 fd
bb54: JSR  $13c0        ; 20 c0 13
bb57: CMP  ($04)        ; d2 04
bb59: .byte $78   ;?
bb5a: TSB  $27          ; 04 27
bb5c: ORA  ($57,X)      ; 01 57
bb5e: BRK               ; 00 
bb5f: LDA  #$00         ; a9 00
bb61: STA  $02          ; 85 02
bb63: LDA  $42          ; a5 42
bb65: STA  $06          ; 85 06
bb67: LDA  $41          ; a5 41
bb69: ORA  #$10         ; 09 10
bb6b: STA  $05          ; 85 05
bb6d: LDA  $40          ; a5 40
bb6f: STA  $04          ; 85 04
bb71: LDA  $43          ; a5 43
bb73: STA  $00          ; 85 00
bb75: LDA  $44          ; a5 44
bb77: ORA  #$10         ; 09 10
bb79: STA  $01          ; 85 01
bb7b: LDA  $45          ; a5 45
bb7d: STA  $02          ; 85 02
bb7f: LDA  #$ff         ; a9 ff
bb81: STA  $aa          ; 85 aa
bb83: LDA  $13b3,X      ; ad b3 13
bb86: ASL               ; 0a 
bb87: ASL               ; 0a 
bb88: ASL               ; 0a 
bb89: STA  $4b          ; 85 4b
bb8b: LDA  $09          ; a5 09
bb8d: AND  #$c7         ; 29 c7
bb8f: ORA  $4b          ; 05 4b
bb91: STA  $09          ; 85 09
bb93: CLI               ; 58 
bb94: RTS               ; 60 
bb95: LDA  #$00         ; a9 00
bb97: STA  $137d,X      ; 8d 7d 13
bb9a: .byte $c2   ;?
bb9b: ORA  ($01,X)      ; 01 01
bb9d: ORA  $10c2,X      ; 0d c2 10
bba0: ORA  ($0d,X)      ; 01 0d
bba2: JSR  $bbd6        ; 20 d6 bb
bba5: CMP  ($01)        ; d2 01
bba7: ORA  ($0d,X)      ; 01 0d
bba9: .byte $27   ;?
bbaa: .byte $93   ;?
bbab: LDA  #$e9         ; a9 e9
bbad: STA  $124e,X      ; 8d 4e 12
bbb0: LDA  #$c2         ; a9 c2
bbb2: STA  $124f,X      ; 8d 4f 12
bbb5: .byte $77   ;?
bbb6: .byte $93   ;?
bbb7: LDA  #$be         ; a9 be
bbb9: STA  $1239,X      ; 8d 39 12
bbbc: LDA  #$b3         ; a9 b3
bbbe: STA  $123a,X      ; 8d 3a 12
bbc1: .byte $f7   ;?
bbc2: .byte $93   ;?
bbc3: .byte $37   ;?
bbc4: .byte $9f   ;?
bbc5: .byte $27   ;?
bbc6: .byte $93   ;?
bbc7: LDA  #$fd         ; a9 fd
bbc9: STA  $124b,X      ; 8d 4b 12
bbcc: LDA  #$bc         ; a9 bc
bbce: STA  $124c,X      ; 8d 4c 12
bbd1: .byte $a7   ;?
bbd2: .byte $93   ;?
bbd3: .byte $47   ;?
bbd4: LDX  #$60         ; a2 60
bbd6: LDA  #$00         ; a9 00
bbd8: STA  $08b7,X      ; 8d b7 08
bbdb: STA  $0f50,X      ; 8d 50 0f
bbde: LDA  #$0f         ; a9 0f
bbe0: STA  $08b8,X      ; 8d b8 08
bbe3: .byte $27   ;?
bbe4: .byte $a3   ;?
bbe5: LDA  $0d03,X      ; ad 03 0d
bbe8: AND  #$02         ; 29 02
bbea: STA  $0d03,X      ; 8d 03 0d
bbed: LDA  #$00         ; a9 00
bbef: STA  $0d00,X      ; 8d 00 0d
bbf2: STA  $137f,X      ; 8d 7f 13
bbf5: STA  $0d05,X      ; 8d 05 0d
bbf8: STA  $0d0c,X      ; 8d 0c 0d
bbfb: STA  $0d0d,X      ; 8d 0d 0d
bbfe: STA  $0d0e,X      ; 8d 0e 0d
bc01: STA  $0d0f,X      ; 8d 0f 0d
bc04: STA  $0e06,X      ; 8d 06 0e
bc07: STA  $0e16,X      ; 8d 16 0e
bc0a: STA  $0e26,X      ; 8d 26 0e
bc0d: STA  $0e36,X      ; 8d 36 0e
bc10: STA  $0e46,X      ; 8d 46 0e
bc13: PHA               ; 48 
bc14: CMP  ($20)        ; d2 20
bc16: ASL  $0e,X        ; 16 0e
bc18: PLA               ; 68 
bc19: PHA               ; 48 
bc1a: CMP  ($20)        ; d2 20
bc1c: ROL  $0e,X        ; 26 0e
bc1e: PLA               ; 68 
bc1f: STA  $0e0c,X      ; 8d 0c 0e
bc22: STA  $0e0d,X      ; 8d 0d 0e
bc25: STA  $0e1c,X      ; 8d 1c 0e
bc28: STA  $0e1d,X      ; 8d 1d 0e
bc2b: STA  $0e2c,X      ; 8d 2c 0e
bc2e: STA  $0e2d,X      ; 8d 2d 0e
bc31: STA  $0e3c,X      ; 8d 3c 0e
bc34: STA  $0e3d,X      ; 8d 3d 0e
bc37: STA  $0e4c,X      ; 8d 4c 0e
bc3a: STA  $0e4d,X      ; 8d 4d 0e
bc3d: STA  $0e0e,X      ; 8d 0e 0e
bc40: STA  $0e1e,X      ; 8d 1e 0e
bc43: STA  $0e2e,X      ; 8d 2e 0e
bc46: STA  $0e3e,X      ; 8d 3e 0e
bc49: STA  $0e4e,X      ; 8d 4e 0e
bc4c: CMP  ($20)        ; d2 20
bc4e: ASL  $d20e        ; 1e 0e d2
bc51: JSR  $0e3e        ; 20 3e 0e
bc54: LDA  #$ff         ; a9 ff
bc56: STA  $0d02,X      ; 8d 02 0d
bc59: STA  $0d04,X      ; 8d 04 0d
bc5c: STA  $0d08,X      ; 8d 08 0d
bc5f: STA  $0d09,X      ; 8d 09 0d
bc62: STA  $0d0a,X      ; 8d 0a 0d
bc65: STA  $0d0b,X      ; 8d 0b 0d
bc68: LDA  #$00         ; a9 00
bc6a: STA  $0d2e,X      ; 8d 2e 0d
bc6d: LDA  #$03         ; a9 03
bc6f: STA  $0d2f,X      ; 8d 2f 0d
bc72: LDA  #$fd         ; a9 fd
bc74: STA  $0e00,X      ; 8d 00 0e
bc77: LDA  #$12         ; a9 12
bc79: STA  $0e01,X      ; 8d 01 0e
bc7c: LDA  #$00         ; a9 00
bc7e: STA  $0e02,X      ; 8d 02 0e
bc81: LDA  #$3d         ; a9 3d
bc83: STA  $0e08,X      ; 8d 08 0e
bc86: LDA  #$13         ; a9 13
bc88: STA  $0e09,X      ; 8d 09 0e
bc8b: LDA  #$00         ; a9 00
bc8d: STA  $0e0a,X      ; 8d 0a 0e
bc90: LDA  #$00         ; a9 00
bc92: STA  $0e10,X      ; 8d 10 0e
bc95: LDA  #$80         ; a9 80
bc97: STA  $0e11,X      ; 8d 11 0e
bc9a: LDA  #$00         ; a9 00
bc9c: STA  $0e12,X      ; 8d 12 0e
bc9f: LDA  #$00         ; a9 00
bca1: STA  $0e18,X      ; 8d 18 0e
bca4: LDA  #$80         ; a9 80
bca6: STA  $0e19,X      ; 8d 19 0e
bca9: LDA  #$00         ; a9 00
bcab: STA  $0e1a,X      ; 8d 1a 0e
bcae: LDA  #$00         ; a9 00
bcb0: STA  $0e20,X      ; 8d 20 0e
bcb3: LDA  #$80         ; a9 80
bcb5: STA  $0e21,X      ; 8d 21 0e
bcb8: LDA  #$00         ; a9 00
bcba: STA  $0e22,X      ; 8d 22 0e
bcbd: LDA  #$8c         ; a9 8c
bcbf: STA  $0e48,X      ; 8d 48 0e
bcc2: LDA  #$13         ; a9 13
bcc4: STA  $0e49,X      ; 8d 49 0e
bcc7: LDA  #$00         ; a9 00
bcc9: STA  $0e4a,X      ; 8d 4a 0e
bccc: .byte $f7   ;?
bccd: LDA  ($e7,X)      ; a1 e7
bccf: LDX  #$07         ; a2 07
bcd1: LDY  #$67         ; a0 67
bcd3: LDA  ($17,X)      ; a1 17
bcd5: LDA  ($27,X)      ; a1 27
bcd7: LDA  ($77,X)      ; a1 77
bcd9: .byte $a3   ;?
bcda: .byte $47   ;?
bcdb: LDA  ($d2,X)      ; a1 d2
bcdd: RTI               ; 40 
bcde: ORA  $0d          ; 05 0d
bce0: CMP  ($40)        ; d2 40
bce2: TSB  $0d          ; 04 0d
bce4: LDA  #$00         ; a9 00
bce6: STA  $1384,X      ; 8d 84 13
bce9: STA  $1385,X      ; 8d 85 13
bcec: STA  $1386,X      ; 8d 86 13
bcef: STA  $1387,X      ; 8d 87 13
bcf2: LDA  #$07         ; a9 07
bcf4: STA  $12d0,X      ; 8d d0 12
bcf7: LDA  #$02         ; a9 02
bcf9: STA  $12d1,X      ; 8d d1 12
bcfc: RTS               ; 60 
bcfd: LDA  #$01         ; a9 01
bcff: STA  $137d,X      ; 8d 7d 13
bd02: CMP  ($10)        ; d2 10
bd04: ORA  ($0d,X)      ; 01 0d
bd06: .byte $27   ;?
bd07: .byte $93   ;?
bd08: LDA  #$25         ; a9 25
bd0a: STA  $124b,X      ; 8d 4b 12
bd0d: LDA  #$bd         ; a9 bd
bd0f: STA  $124c,X      ; 8d 4c 12
bd12: .byte $a7   ;?
bd13: .byte $93   ;?
bd14: BRA  $bd24        ; 80 0e
bd16: .byte $27   ;?
bd17: .byte $93   ;?
bd18: LDA  #$95         ; a9 95
bd1a: STA  $124b,X      ; 8d 4b 12
bd1d: LDA  #$bb         ; a9 bb
bd1f: STA  $124c,X      ; 8d 4c 12
bd22: .byte $a7   ;?
bd23: .byte $93   ;?
bd24: RTS               ; 60 
bd25: LDA  #$02         ; a9 02
bd27: STA  $137d,X      ; 8d 7d 13
bd2a: .byte $27   ;?
bd2b: .byte $93   ;?
bd2c: CMP  ($80)        ; d2 80
bd2e: .byte $02   ;?
bd2f: ORA  $80d2,X      ; 0d d2 80
bd32: .byte $03   ;?
bd33: ORA  $a960,X      ; 0d 60 a9
bd36: .byte $03   ;?
bd37: STA  $137d,X      ; 8d 7d 13
bd3a: SBC  ($03)        ; f2 03
bd3c: ORA  $1601,X      ; 0d 01 16
bd3f: .byte $27   ;?
bd40: .byte $93   ;?
bd41: CMP  ($01)        ; d2 01
bd43: ASL  $0e,X        ; 06 0e
bd45: CMP  ($01)        ; d2 01
bd47: TSB  ($d20d)      ; 0c 0d d2
bd4a: ORA  ($0e,X)      ; 01 0e
bd4c: ORA  $80d2,X      ; 0d d2 80
bd4f: .byte $02   ;?
bd50: ORA  $80d2,X      ; 0d d2 80
bd53: .byte $03   ;?
bd54: ORA  $a960,X      ; 0d 60 a9
bd57: TSB  $8d          ; 04 8d
bd59: ADC  $2713,X      ; 7d 13 27
bd5c: .byte $a3   ;?
bd5d: .byte $6f   ;?
bd5e: LDA  ($11,X)      ; a1 11
bd60: .byte $67   ;?
bd61: LDA  ($ad,X)      ; a1 ad
bd63: .byte $7f   ;?
bd64: .byte $13   ;?
bd65: BEQ  $bd71        ; f0 0a
bd67: CMP  $0d00,X      ; cd 00 0d
bd6a: BEQ  $bd6f        ; f0 03
bd6c: STA  $0d00,X      ; 8d 00 0d
bd6f: .byte $27   ;?
bd70: .byte $93   ;?
bd71: RTS               ; 60 
bd72: LDA  #$05         ; a9 05
bd74: STA  $137d,X      ; 8d 7d 13
bd77: .byte $a7   ;?
bd78: .byte $a3   ;?
bd79: CMP  ($01)        ; d2 01
bd7b: ASL  $0e,X        ; 06 0e
bd7d: CMP  ($01)        ; d2 01
bd7f: TSB  ($d20d)      ; 0c 0d d2
bd82: ORA  ($0e,X)      ; 01 0e
bd84: ORA  $01d2,X      ; 0d d2 01
bd87: ASL  $0e,X        ; 16 0e
bd89: CMP  ($02)        ; d2 02
bd8b: TSB  ($d20d)      ; 0c 0d d2
bd8e: .byte $02   ;?
bd8f: ASL  $d20d        ; 0e 0d d2
bd92: ORA  ($26,X)      ; 01 26
bd94: ASL  $04d2        ; 0e d2 04
bd97: TSB  ($d20d)      ; 0c 0d d2
bd9a: TSB  $0e          ; 04 0e
bd9c: ORA  $01d2,X      ; 0d d2 01
bd9f: ROL  $0e,X        ; 36 0e
bda1: CMP  ($08)        ; d2 08
bda3: TSB  ($d20d)      ; 0c 0d d2
bda6: PHP               ; 08 
bda7: ASL  $d20d        ; 0e 0d d2
bdaa: BPL  $bdba        ; 10 0e
bdac: ORA  $9327,X      ; 0d 27 93
bdaf: CMP  ($02)        ; d2 02
bdb1: .byte $02   ;?
bdb2: ORA  $02d2,X      ; 0d d2 02
bdb5: ORA  ($0d,X)      ; 01 0d
bdb7: CMP  ($02)        ; d2 02
bdb9: .byte $03   ;?
bdba: ORA  $a24f,X      ; 0d 4f a2
bdbd: .byte $02   ;?
bdbe: .byte $47   ;?
bdbf: LDX  #$60         ; a2 60
bdc1: JSR  $1426        ; 20 26 14
bdc4: LDA  #$00         ; a9 00
bdc6: STA  $08b7,X      ; 8d b7 08
bdc9: STA  $0f50,X      ; 8d 50 0f
bdcc: LDA  #$0f         ; a9 0f
bdce: STA  $08b8,X      ; 8d b8 08
bdd1: CMP  ($08)        ; d2 08
bdd3: .byte $02   ;?
bdd4: ORA  $04d2,X      ; 0d d2 04
bdd7: .byte $02   ;?
bdd8: ORA  $80d2,X      ; 0d d2 80
bddb: .byte $02   ;?
bddc: ORA  $80d2,X      ; 0d d2 80
bddf: .byte $03   ;?
bde0: ORA  $10d2,X      ; 0d d2 10
bde3: .byte $02   ;?
bde4: ORA  $8aae,X      ; 0d ae 8a
bde7: .byte $13   ;?
bde8: JSR  $a4d8        ; 20 d8 a4
bdeb: .byte $ff   ;?
bdec: ORA  ($03,X)      ; 01 03
bdee: JMP  ($be37)      ; 4c 37 be
bdf1: .byte $1f   ;?
bdf2: LDX  #$02         ; a2 02
bdf4: .byte $17   ;?
bdf5: LDX  #$2f         ; a2 2f
bdf7: LDX  #$02         ; a2 02
bdf9: .byte $27   ;?
bdfa: LDX  #$c2         ; a2 c2
bdfc: .byte $02   ;?
bdfd: ORA  ($0d,X)      ; 01 0d
bdff: .byte $c7   ;?
be00: LDX  #$d2         ; a2 d2
be02: JSR  $0f7d        ; 20 7d 0f
be05: JSR  $baa9        ; 20 a9 ba
be08: .byte $e2   ;?
be09: .byte $02   ;?
be0a: ORA  $1108,X      ; 0d 08 11
be0d: CMP  ($08)        ; d2 08
be0f: .byte $02   ;?
be10: ORA  $04d2,X      ; 0d d2 04
be13: ORA  ($0d,X)      ; 01 0d
be15: JSR  $a4e3        ; 20 e3 a4
be18: .byte $c2   ;?
be19: TSB  $01          ; 04 01
be1b: ORA  $0480,X      ; 0d 80 04
be1e: CMP  ($04)        ; d2 04
be20: .byte $02   ;?
be21: ORA  $0020,X      ; 0d 20 00
be24: .byte $bf   ;?
be25: CMP  ($02)        ; d2 02
be27: .byte $02   ;?
be28: ORA  $84ad,X      ; 0d ad 84
be2b: .byte $13   ;?
be2c: BEQ  $be36        ; f0 08
be2e: CMP  ($02)        ; d2 02
be30: ORA  ($0d,X)      ; 01 0d
be32: CMP  ($02)        ; d2 02
be34: .byte $03   ;?
be35: ORA  $2060,X      ; 0d 60 20
be38: .byte $57   ;?
be39: LDX  $6060        ; be 60 60
be3c: .byte $ff   ;?
be3d: ORA  ($02,X)      ; 01 02
be3f: BRA  $be57        ; 80 16
be41: LDA  $137d,X      ; ad 7d 13
be44: CMP  #$00         ; c9 00
be46: BNE  $be56        ; d0 0e
be48: .byte $27   ;?
be49: .byte $93   ;?
be4a: LDA  #$fd         ; a9 fd
be4c: STA  $124b,X      ; 8d 4b 12
be4f: LDA  #$bc         ; a9 bc
be51: STA  $124c,X      ; 8d 4c 12
be54: .byte $a7   ;?
be55: .byte $93   ;?
be56: RTS               ; 60 
be57: .byte $c2   ;?
be58: ORA  ($01,X)      ; 01 01
be5a: ORA  $10c2,X      ; 0d c2 10
be5d: ORA  ($0d,X)      ; 01 0d
be5f: LDA  #$00         ; a9 00
be61: STA  $1386,X      ; 8d 86 13
be64: STA  $1387,X      ; 8d 87 13
be67: JSR  $d564        ; 20 64 d5
be6a: LDA  #$00         ; a9 00
be6c: STA  $137d,X      ; 8d 7d 13
be6f: .byte $27   ;?
be70: .byte $93   ;?
be71: LDA  #$95         ; a9 95
be73: STA  $124b,X      ; 8d 4b 12
be76: LDA  #$bb         ; a9 bb
be78: STA  $124c,X      ; 8d 4c 12
be7b: .byte $a7   ;?
be7c: .byte $93   ;?
be7d: .byte $c2   ;?
be7e: .byte $02   ;?
be7f: .byte $03   ;?
be80: ORA  $80c2,X      ; 0d c2 80
be83: .byte $03   ;?
be84: ORA  $a327,X      ; 0d 27 a3
be87: .byte $9f   ;?
be88: .byte $4f   ;?
be89: TSB  ($6bad)      ; 0c ad 6b
be8c: ORA  ($8d)        ; 12 8d
be8e: ASL               ; 0a 
be8f: BPL  $be3e        ; 10 ad
be91: ADC  $8d12,X      ; 6d 12 8d
be94: .byte $0b   ;?
be95: BPL  $bef7        ; 10 60
be97: JSR  $141d        ; 20 1d 14
be9a: .byte $e2   ;?
be9b: .byte $03   ;?
be9c: ORA  $2680,X      ; 0d 80 26
be9f: .byte $e2   ;?
bea0: .byte $02   ;?
bea1: ORA  $2180,X      ; 0d 80 21
bea4: CMP  ($02)        ; d2 02
bea6: .byte $02   ;?
bea7: ORA  $80c2,X      ; 0d c2 80
beaa: .byte $03   ;?
beab: ORA  $d620,X      ; 0d 20 d6
beae: .byte $bb   ;?
beaf: CMP  ($01)        ; d2 01
beb1: .byte $02   ;?
beb2: ORA  $01d2,X      ; 0d d2 01
beb5: .byte $03   ;?
beb6: ORA  $9327,X      ; 0d 27 93
beb9: LDA  #$35         ; a9 35
bebb: STA  $124b,X      ; 8d 4b 12
bebe: LDA  #$bd         ; a9 bd
bec0: STA  $124c,X      ; 8d 4c 12
bec3: .byte $a7   ;?
bec4: .byte $93   ;?
bec5: RTS               ; 60 
bec6: JSR  $1420        ; 20 20 14
bec9: .byte $e2   ;?
beca: .byte $03   ;?
becb: ORA  $0901,X      ; 0d 01 09
bece: .byte $e2   ;?
becf: .byte $02   ;?
bed0: ORA  $0401,X      ; 0d 01 04
bed3: .byte $c2   ;?
bed4: ORA  ($03,X)      ; 01 03
bed6: ORA  $2060,X      ; 0d 60 20
bed9: .byte $23   ;?
beda: TRB  $e2          ; 14 e2
bedc: .byte $03   ;?
bedd: ORA  $1f02,X      ; 0d 02 1f
bee0: .byte $e2   ;?
bee1: .byte $02   ;?
bee2: ORA  $1a02,X      ; 0d 02 1a
bee5: .byte $c2   ;?
bee6: .byte $02   ;?
bee7: .byte $03   ;?
bee8: ORA  $02d2,X      ; 0d d2 02
beeb: .byte $02   ;?
beec: ORA  $08d2,X      ; 0d d2 08
beef: .byte $02   ;?
bef0: ORA  $04d2,X      ; 0d d2 04
bef3: .byte $02   ;?
bef4: ORA  $02c2,X      ; 0d c2 02
bef7: .byte $03   ;?
bef8: ORA  $80c2,X      ; 0d c2 80
befb: .byte $03   ;?
befc: ORA  $9fc7,X      ; 0d c7 9f
beff: RTS               ; 60 
bf00: JSR  $1429        ; 20 29 14
bf03: .byte $c2   ;?
bf04: PHP               ; 08 
bf05: .byte $03   ;?
bf06: ORA  $04c2,X      ; 0d c2 04
bf09: .byte $03   ;?
bf0a: ORA  $e260,X      ; 0d 60 e2
bf0d: TSB  $0d          ; 04 0d
bf0f: BRA  $bf1d        ; 80 0c
bf11: CMP  ($80)        ; d2 80
bf13: TSB  $0d          ; 04 0d
bf15: .byte $c2   ;?
bf16: BRA  $bf1d        ; 80 05
bf18: ORA  $01d2,X      ; 0d d2 01
bf1b: TSB  ($600d)      ; 0c 0d 60
bf1e: JSR  $142c        ; 20 2c 14
bf21: .byte $e2   ;?
bf22: ORA  $0d          ; 05 0d
bf24: RTI               ; 40 
bf25: AND  $e2          ; 25 e2
bf27: TSB  $0d          ; 04 0d
bf29: RTI               ; 40 
bf2a: JSR  $40d2        ; 20 d2 40
bf2d: TSB  $0d          ; 04 0d
bf2f: .byte $e2   ;?
bf30: BVS  $bf41        ; 70 0f
bf32: BRA  $bf3c        ; 80 08
bf34: LDA  $d2          ; a5 d2
bf36: CMP  #$ff         ; c9 ff
bf38: BEQ  $bf3c        ; f0 02
bf3a: INC  $d2,X        ; e6 d2
bf3c: .byte $6f   ;?
bf3d: .byte $a3   ;?
bf3e: PHP               ; 08 
bf3f: LDA  $d4          ; a5 d4
bf41: CMP  #$ff         ; c9 ff
bf43: BEQ  $bf47        ; f0 02
bf45: INC  $d4,X        ; e6 d4
bf47: LDA  #$01         ; a9 01
bf49: STA  $dc          ; 85 dc
bf4b: RTS               ; 60 
bf4c: RTS               ; 60 
bf4d: LDA  $0d08,X      ; ad 08 0d
bf50: BNE  $bf53        ; d0 01
bf52: RTS               ; 60 
bf53: JSR  $142f        ; 20 2f 14
bf56: .byte $e2   ;?
bf57: PHP               ; 08 
bf58: ORA  $4201,X      ; 0d 01 42
bf5b: LDA  #$01         ; a9 01
bf5d: STA  $0d08,X      ; 8d 08 0d
bf60: SBC  ($06)        ; f2 06
bf62: ASL  $3801        ; 0e 01 38
bf65: .byte $e2   ;?
bf66: ASL  $0e,X        ; 06 0e
bf68: BRA  $bf7a        ; 80 10
bf6a: .byte $37   ;?
bf6b: .byte $9f   ;?
bf6c: LDA  #$e9         ; a9 e9
bf6e: STA  $124e,X      ; 8d 4e 12
bf71: LDA  #$c2         ; a9 c2
bf73: STA  $124f,X      ; 8d 4f 12
bf76: .byte $b7   ;?
bf77: .byte $9f   ;?
bf78: BRA  $bf9d        ; 80 23
bf7a: .byte $e2   ;?
bf7b: ASL  $0e,X        ; 06 0e
bf7d: PHP               ; 08 
bf7e: BPL  $bfb7        ; 10 37
bf80: .byte $9f   ;?
bf81: LDA  #$43         ; a9 43
bf83: STA  $124e,X      ; 8d 4e 12
bf86: LDA  #$c3         ; a9 c3
bf88: STA  $124f,X      ; 8d 4f 12
bf8b: .byte $b7   ;?
bf8c: .byte $9f   ;?
bf8d: BRA  $bf9d        ; 80 0e
bf8f: .byte $37   ;?
bf90: .byte $9f   ;?
bf91: LDA  #$8a         ; a9 8a
bf93: STA  $124e,X      ; 8d 4e 12
bf96: LDA  #$c9         ; a9 c9
bf98: STA  $124f,X      ; 8d 4f 12
bf9b: .byte $b7   ;?
bf9c: .byte $9f   ;?
bf9d: SBC  ($08)        ; f2 08
bf9f: ORA  $0302,X      ; 0d 02 03
bfa2: JMP  ($c0e5)      ; 4c e5 c0
bfa5: LDA  #$02         ; a9 02
bfa7: STA  $0d08,X      ; 8d 08 0d
bfaa: SBC  ($16)        ; f2 16
bfac: ASL  $f301        ; 0e 01 f3
bfaf: .byte $e2   ;?
bfb0: ASL  $0e,X        ; 16 0e
bfb2: BRA  $bfbe        ; 80 0a
bfb4: LDA  #$43         ; a9 43
bfb6: STA  $38          ; 85 38
bfb8: JSR  $c307        ; 20 07 c3
bfbb: JMP  ($c0e5)      ; 4c e5 c0
bfbe: SBC  ($70)        ; f2 70
bfc0: .byte $0f   ;?
bfc1: BRA  $bfc9        ; 80 06
bfc3: CMP  ($80)        ; d2 80
bfc5: BVS  $bfd6        ; 70 0f
bfc7: BRA  $bfde        ; 80 15
bfc9: LDA  $13b1,X      ; ad b1 13
bfcc: BIT  #$01         ; 89 01
bfce: CMP  $0d06,X      ; cd 06 0d
bfd1: BEQ  $bfde        ; f0 0b
bfd3: LDA  #$4d         ; a9 4d
bfd5: JSR  $bf4c        ; 20 4c bf
bfd8: JSR  $c307        ; 20 07 c3
bfdb: JMP  ($c0e5)      ; 4c e5 c0
bfde: LDA  $0d06,X      ; ad 06 0d
bfe1: STA  $13b1,X      ; 8d b1 13
bfe4: LDA  ($00)        ; b2 00
bfe6: CMP  ($ad)        ; d2 ad
bfe8: .byte $87   ;?
bfe9: .byte $13   ;?
bfea: CMP  #$02         ; c9 02
bfec: BEQ  $c00e        ; f0 20
bfee: LDX  #$1e         ; a2 1e
bff0: LDA  $f624,X      ; bd 24 f6
bff3: CMP  $0e14,X      ; cd 14 0e
bff6: BNE  $c005        ; d0 0d
bff8: LDA  $f625,X      ; bd 25 f6
bffb: CMP  $0e15,X      ; cd 15 0e
bffe: BNE  $c005        ; d0 05

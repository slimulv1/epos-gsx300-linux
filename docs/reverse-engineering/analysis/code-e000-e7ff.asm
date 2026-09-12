e000: ROL  $ad,X        ; 26 ad
e002: ASL  $10,X        ; 16 10
e004: AND  #$0f         ; 29 0f
e006: STA  $40          ; 85 40
e008: LDA  $0f2a,X      ; ad 2a 0f
e00b: AND  #$0f         ; 29 0f
e00d: CMP  $40          ; c5 40
e00f: BNE  $e028        ; d0 17
e011: LDA  $0f2b,X      ; ad 2b 0f
e014: AND  #$0f         ; 29 0f
e016: CMP  $40          ; c5 40
e018: BNE  $e028        ; d0 0e
e01a: LDA  $0f20,X      ; ad 20 0f
e01d: AND  #$3c         ; 29 3c
e01f: LSR               ; 4a 
e020: LSR               ; 4a 
e021: CMP  #$00         ; c9 00
e023: CMP  $40          ; c5 40
e025: BNE  $e028        ; d0 01
e027: RTS               ; 60 
e028: LDA  $40          ; a5 40
e02a: STA  $4b          ; 85 4b
e02c: LDA  $0f2a,X      ; ad 2a 0f
e02f: AND  #$f0         ; 29 f0
e031: ORA  $4b          ; 05 4b
e033: STA  $0f2a,X      ; 8d 2a 0f
e036: LDA  $40          ; a5 40
e038: STA  $4b          ; 85 4b
e03a: LDA  $0f2b,X      ; ad 2b 0f
e03d: AND  #$f0         ; 29 f0
e03f: ORA  $4b          ; 05 4b
e041: STA  $0f2b,X      ; 8d 2b 0f
e044: LDA  $40          ; a5 40
e046: ASL               ; 0a 
e047: ASL               ; 0a 
e048: STA  $4b          ; 85 4b
e04a: LDA  $0f20,X      ; ad 20 0f
e04d: AND  #$c3         ; 29 c3
e04f: ORA  $4b          ; 05 4b
e051: STA  $0f20,X      ; 8d 20 0f
e054: LDA  ($0a)        ; b2 0a
e056: .byte $af   ;?
e057: .byte $27   ;?
e058: STY  $60,X        ; 94 60
e05a: INA               ; 1a 
e05b: STA  $40          ; 85 40
e05d: TYA               ; 98 
e05e: INA               ; 1a 
e05f: ASL  $40,X        ; 06 40
e061: ROL               ; 2a 
e062: STA  $40          ; 85 40
e064: LDA  ($ab)        ; b2 ab
e066: TSB  $20          ; 44 20
e068: TAX               ; aa 
e069: CMP  #$a5         ; c9 a5
e06b: EOR  ($4a,X)      ; 41 4a
e06d: RTS               ; 60 
e06e: BMI  $e078        ; 30 08
e070: CMP  #$06         ; c9 06
e072: BCC  $e07e        ; 90 0a
e074: LDA  #$05         ; a9 05
e076: BRA  $e07e        ; 80 06
e078: CMP  #$b6         ; c9 b6
e07a: BCS  $e07e        ; b0 02
e07c: LDA  #$b6         ; a9 b6
e07e: SEC               ; 38 
e07f: SBC  #$05         ; e9 05
e081: RTS               ; 60 
e082: .byte $2f   ;?
e083: .byte $a3   ;?
e084: BPL  $e00b        ; 10 85
e086: RTI               ; 40 
e087: TYA               ; 98 
e088: JSR  $e06e        ; 20 6e e0
e08b: STZ  $40          ; 64 40
e08d: BEQ  $e095        ; f0 06
e08f: CMP  #$b1         ; c9 b1
e091: BCS  $e095        ; b0 02
e093: LDA  #$b1         ; a9 b1
e095: RTS               ; 60 
e096: .byte $2f   ;?
e097: .byte $a3   ;?
e098: TSB  ($4faf)      ; 0c af 4f
e09b: ORA  #$c4         ; 09 c4
e09d: ADC  ($d0,X)      ; 61 d0
e09f: .byte $02   ;?
e0a0: LDY  #$80         ; a0 80
e0a2: JSR  $e082        ; 20 82 e0
e0a5: RTS               ; 60 
e0a6: JSR  $13ff        ; 20 ff 13
e0a9: SBC  ($13)        ; f2 13
e0ab: BPL  $e02d        ; 10 80
e0ad: .byte $03   ;?
e0ae: JMP  ($e129)      ; 4c 29 e1
e0b1: .byte $df   ;?
e0b2: STY  $01,X        ; 94 01
e0b4: RTS               ; 60 
e0b5: .byte $4f   ;?
e0b6: EOR  $0f27,X      ; 4d 27 0f
e0b9: STA  $f224,X      ; 9d 24 f2
e0bc: INA               ; 1a 
e0bd: .byte $0f   ;?
e0be: BRA  $e0c4        ; 80 04
e0c0: .byte $27   ;?
e0c1: TXS               ; 9a 
e0c2: BRA  $e0c6        ; 80 02
e0c4: .byte $a7   ;?
e0c5: TXS               ; 9a 
e0c6: .byte $c2   ;?
e0c7: PHP               ; 08 
e0c8: INA               ; 1a 
e0c9: .byte $0f   ;?
e0ca: .byte $2f   ;?
e0cb: EOR  $f20d,X      ; 4d 0d f2
e0ce: .byte $c7   ;?
e0cf: TSB  $08          ; 04 08
e0d1: PHP               ; 08 
e0d2: LDA  $0f44,X      ; ad 44 0f
e0d5: AND  #$3f         ; 29 3f
e0d7: STA  $0f44,X      ; 8d 44 0f
e0da: JSR  $148c        ; 20 8c 14
e0dd: .byte $07   ;?
e0de: STA  $9acf,X      ; 9d cf 9a
e0e1: .byte $19   ;?
e0e2: JSR  $e18d        ; 20 8d e1
e0e5: LDA  $0f18,X      ; ad 18 0f
e0e8: AND  #$03         ; 29 03
e0ea: BEQ  $e0f7        ; f0 0b
e0ec: LDA  $0f71,X      ; ad 71 0f
e0ef: AND  #$03         ; 29 03
e0f1: BEQ  $e0f7        ; f0 04
e0f3: JSR  $e167        ; 20 67 e1
e0f6: RTS               ; 60 
e0f7: JSR  $e174        ; 20 74 e1
e0fa: RTS               ; 60 
e0fb: .byte $4f   ;?
e0fc: .byte $4f   ;?
e0fd: .byte $0f   ;?
e0fe: SBC  ($14)        ; f2 14
e100: BPL  $e082        ; 10 80
e102: .byte $07   ;?
e103: SBC  ($14)        ; f2 14
e105: BPL  $e108        ; 10 01
e107: .byte $13   ;?
e108: BRA  $e10d        ; 80 03
e10a: .byte $ff   ;?
e10b: TXS               ; 9a 
e10c: ASL  $18ad        ; 0e ad 18
e10f: .byte $0f   ;?
e110: AND  #$03         ; 29 03
e112: BEQ  $e122        ; f0 0e
e114: LDA  $0f71,X      ; ad 71 0f
e117: AND  #$03         ; 29 03
e119: BEQ  $e122        ; f0 07
e11b: JSR  $e17d        ; 20 7d e1
e11e: JSR  $e167        ; 20 67 e1
e121: RTS               ; 60 
e122: JSR  $e18d        ; 20 8d e1
e125: JSR  $e174        ; 20 74 e1
e128: RTS               ; 60 
e129: .byte $4f   ;?
e12a: .byte $4f   ;?
e12b: .byte $0f   ;?
e12c: SBC  ($14)        ; f2 14
e12e: BPL  $e0b0        ; 10 80
e130: .byte $07   ;?
e131: SBC  ($14)        ; f2 14
e133: BPL  $e136        ; 10 01
e135: .byte $13   ;?
e136: BRA  $e13b        ; 80 03
e138: .byte $ff   ;?
e139: TXS               ; 9a 
e13a: ASL  $18ad        ; 0e ad 18
e13d: .byte $0f   ;?
e13e: AND  #$03         ; 29 03
e140: BEQ  $e164        ; f0 22
e142: LDA  $0f71,X      ; ad 71 0f
e145: AND  #$03         ; 29 03
e147: BEQ  $e164        ; f0 1b
e149: .byte $e2   ;?
e14a: .byte $13   ;?
e14b: BPL  $e14e        ; 10 01
e14d: ORA  $20          ; 05 20
e14f: ADC  $80e1,X      ; 7d e1 80
e152: .byte $03   ;?
e153: JSR  $e18d        ; 20 8d e1
e156: .byte $e2   ;?
e157: .byte $13   ;?
e158: BPL  $e15c        ; 10 02
e15a: ORA  $20          ; 05 20
e15c: .byte $67   ;?
e15d: SBC  ($80,X)      ; e1 80
e15f: .byte $03   ;?
e160: JSR  $e174        ; 20 74 e1
e163: RTS               ; 60 
e164: JMP  ($e174)      ; 4c 74 e1
e167: LDA  #$00         ; a9 00
e169: .byte $e2   ;?
e16a: .byte $13   ;?
e16b: BPL  $e18d        ; 10 20
e16d: .byte $02   ;?
e16e: ORA  #$88         ; 09 88
e170: STA  $0f1b,X      ; 8d 1b 0f
e173: RTS               ; 60 
e174: LDA  $0f1b,X      ; ad 1b 0f
e177: ORA  #$33         ; 09 33
e179: STA  $0f1b,X      ; 8d 1b 0f
e17c: RTS               ; 60 
e17d: JSR  $145c        ; 20 5c 14
e180: CMP  ($04)        ; d2 04
e182: .byte $3f   ;?
e183: .byte $0f   ;?
e184: LDA  $0f1c,X      ; ad 1c 0f
e187: ORA  #$50         ; 09 50
e189: STA  $0f1c,X      ; 8d 1c 0f
e18c: RTS               ; 60 
e18d: JSR  $145f        ; 20 5f 14
e190: LDA  $0f1c,X      ; ad 1c 0f
e193: AND  #$af         ; 29 af
e195: STA  $0f1c,X      ; 8d 1c 0f
e198: RTS               ; 60 
e199: JSR  $1402        ; 20 02 14
e19c: JSR  $e3af        ; 20 af e3
e19f: SBC  ($70)        ; f2 70
e1a1: .byte $0f   ;?
e1a2: ORA  ($0b,X)      ; 01 0b
e1a4: JSR  $e221        ; 20 21 e2
e1a7: LDA  #$00         ; a9 00
e1a9: STA  $0f23,X      ; 8d 23 0f
e1ac: JMP  ($e410)      ; 4c 10 e4
e1af: .byte $e2   ;?
e1b0: ASL  $10,X        ; 16 10
e1b2: BRA  $e1bf        ; 80 0b
e1b4: LDA  #$11         ; a9 11
e1b6: STA  $0f23,X      ; 8d 23 0f
e1b9: JSR  $e1ca        ; 20 ca e1
e1bc: JMP  ($e410)      ; 4c 10 e4
e1bf: LDA  #$11         ; a9 11
e1c1: STA  $0f23,X      ; 8d 23 0f
e1c4: JSR  $e221        ; 20 21 e2
e1c7: JMP  ($e22f)      ; 4c 2f e2
e1ca: LDA  #$01         ; a9 01
e1cc: ASL               ; 0a 
e1cd: ASL               ; 0a 
e1ce: ASL               ; 0a 
e1cf: ASL               ; 0a 
e1d0: ASL               ; 0a 
e1d1: ASL               ; 0a 
e1d2: STA  $4b          ; 85 4b
e1d4: LDA  $0f1d,X      ; ad 1d 0f
e1d7: AND  #$3f         ; 29 3f
e1d9: ORA  $4b          ; 05 4b
e1db: STA  $0f1d,X      ; 8d 1d 0f
e1de: SBC  ($15)        ; f2 15
e1e0: BPL  $e162        ; 10 80
e1e2: ASL               ; 0a 
e1e3: .byte $c2   ;?
e1e4: BRA  $e20a        ; 80 24
e1e6: .byte $0f   ;?
e1e7: .byte $c2   ;?
e1e8: BRA  $e20f        ; 80 25
e1ea: .byte $0f   ;?
e1eb: BRA  $e1f5        ; 80 08
e1ed: CMP  ($80)        ; d2 80
e1ef: BIT  $0f          ; 24 0f
e1f1: CMP  ($80)        ; d2 80
e1f3: AND  $0f          ; 25 0f
e1f5: CMP  ($40)        ; d2 40
e1f7: BIT  $0f          ; 24 0f
e1f9: CMP  ($40)        ; d2 40
e1fb: AND  $0f          ; 25 0f
e1fd: LDA  $1015,X      ; ad 15 10
e200: AND  #$0f         ; 29 0f
e202: PHA               ; 48 
e203: ASL               ; 0a 
e204: ASL               ; 0a 
e205: STA  $4b          ; 85 4b
e207: LDA  $0f21,X      ; ad 21 0f
e20a: AND  #$c3         ; 29 c3
e20c: ORA  $4b          ; 05 4b
e20e: STA  $0f21,X      ; 8d 21 0f
e211: PLA               ; 68 
e212: ASL               ; 0a 
e213: ASL               ; 0a 
e214: STA  $4b          ; 85 4b
e216: LDA  $0f21,X      ; ad 21 0f
e219: AND  #$c3         ; 29 c3
e21b: ORA  $4b          ; 05 4b
e21d: STA  $0f21,X      ; 8d 21 0f
e220: RTS               ; 60 
e221: LDA  $0f24,X      ; ad 24 0f
e224: AND  #$bf         ; 29 bf
e226: ORA  #$80         ; 09 80
e228: STA  $0f24,X      ; 8d 24 0f
e22b: STA  $0f25,X      ; 8d 25 0f
e22e: RTS               ; 60 
e22f: JSR  $1477        ; 20 77 14
e232: .byte $e2   ;?
e233: .byte $17   ;?
e234: BPL  $e1b6        ; 10 80
e236: .byte $22   ;?
e237: LDA  #$00         ; a9 00
e239: ASL               ; 0a 
e23a: ASL               ; 0a 
e23b: ASL               ; 0a 
e23c: ASL               ; 0a 
e23d: ASL               ; 0a 
e23e: ASL               ; 0a 
e23f: STA  $4b          ; 85 4b
e241: LDA  $0f1d,X      ; ad 1d 0f
e244: AND  #$3f         ; 29 3f
e246: ORA  $4b          ; 05 4b
e248: STA  $0f1d,X      ; 8d 1d 0f
e24b: CMP  ($01)        ; d2 01
e24d: EOR  ($08,X)      ; 41 08
e24f: CMP  ($02)        ; d2 02
e251: EOR  ($08,X)      ; 41 08
e253: JSR  $e44c        ; 20 4c e4
e256: JMP  ($e41e)      ; 4c 1e e4
e259: JSR  $e491        ; 20 91 e4
e25c: .byte $2f   ;?
e25d: EOR  $f254,X      ; 4d 54 f2
e260: .byte $c7   ;?
e261: TSB  $08          ; 04 08
e263: ORA  #$d2         ; 09 d2
e265: .byte $02   ;?
e266: JSR  $c20f        ; 20 0f c2
e269: ORA  ($20,X)      ; 01 20
e26b: .byte $0f   ;?
e26c: RTS               ; 60 
e26d: SBC  ($20)        ; f2 20
e26f: .byte $0f   ;?
e270: ORA  ($40,X)      ; 01 40
e272: LDA  #$03         ; a9 03
e274: SBC  ($c7)        ; f2 c7
e276: TSB  $04          ; 04 04
e278: .byte $02   ;?
e279: LDA  #$02         ; a9 02
e27b: ASL               ; 0a 
e27c: ASL               ; 0a 
e27d: ASL               ; 0a 
e27e: ASL               ; 0a 
e27f: ASL               ; 0a 
e280: ASL               ; 0a 
e281: STA  $4b          ; 85 4b
e283: LDA  $0f1d,X      ; ad 1d 0f
e286: AND  #$3f         ; 29 3f
e288: ORA  $4b          ; 05 4b
e28a: STA  $0f1d,X      ; 8d 1d 0f
e28d: LDA  #$00         ; a9 00
e28f: ASL               ; 0a 
e290: ASL               ; 0a 
e291: STA  $4b          ; 85 4b
e293: LDA  $0f20,X      ; ad 20 0f
e296: AND  #$c3         ; 29 c3
e298: ORA  $4b          ; 05 4b
e29a: STA  $0f20,X      ; 8d 20 0f
e29d: CMP  ($80)        ; d2 80
e29f: .byte $1f   ;?
e2a0: .byte $0f   ;?
e2a1: CMP  ($80)        ; d2 80
e2a3: ASL  $c20f        ; 1e 0f c2
e2a6: .byte $02   ;?
e2a7: JSR  $d20f        ; 20 0f d2
e2aa: ORA  ($20,X)      ; 01 20
e2ac: .byte $0f   ;?
e2ad: LDA  ($19)        ; b2 19
e2af: .byte $af   ;?
e2b0: .byte $27   ;?
e2b1: STY  $60,X        ; 94 60
e2b3: LDA  #$00         ; a9 00
e2b5: ASL               ; 0a 
e2b6: ASL               ; 0a 
e2b7: ASL               ; 0a 
e2b8: ASL               ; 0a 
e2b9: ASL               ; 0a 
e2ba: ASL               ; 0a 
e2bb: STA  $4b          ; 85 4b
e2bd: LDA  $0f1d,X      ; ad 1d 0f
e2c0: AND  #$3f         ; 29 3f
e2c2: ORA  $4b          ; 05 4b
e2c4: STA  $0f1d,X      ; 8d 1d 0f
e2c7: SBC  ($2a)        ; f2 2a
e2c9: .byte $0f   ;?
e2ca: BPL  $e2f8        ; 10 2c
e2cc: .byte $2f   ;?
e2cd: STY  $29,X        ; 94 29
e2cf: LDA  #$00         ; a9 00
e2d1: STA  $4b          ; 85 4b
e2d3: LDA  $0f2a,X      ; ad 2a 0f
e2d6: AND  #$f0         ; 29 f0
e2d8: ORA  $4b          ; 05 4b
e2da: STA  $0f2a,X      ; 8d 2a 0f
e2dd: LDA  #$00         ; a9 00
e2df: STA  $4b          ; 85 4b
e2e1: LDA  $0f2b,X      ; ad 2b 0f
e2e4: AND  #$f0         ; 29 f0
e2e6: ORA  $4b          ; 05 4b
e2e8: STA  $0f2b,X      ; 8d 2b 0f
e2eb: CMP  ($80)        ; d2 80
e2ed: .byte $1f   ;?
e2ee: .byte $0f   ;?
e2ef: CMP  ($80)        ; d2 80
e2f1: ASL  $b20f        ; 1e 0f b2
e2f4: .byte $19   ;?
e2f5: .byte $af   ;?
e2f6: .byte $27   ;?
e2f7: STY  $f2,X        ; 94 f2
e2f9: ASL  $10,X        ; 16 10
e2fb: RTI               ; 40 
e2fc: ASL  $c2,X        ; 06 c2
e2fe: JSR  $0f2a        ; 20 2a 0f
e301: BRA  $e34e        ; 80 4b
e303: SBC  ($2a)        ; f2 2a
e305: .byte $0f   ;?
e306: JSR  $d217        ; 20 17 d2
e309: BRA  $e329        ; 80 1e
e30b: .byte $0f   ;?
e30c: LDA  #$01         ; a9 01
e30e: STA  $4b          ; 85 4b
e310: LDA  $0f2a,X      ; ad 2a 0f
e313: AND  #$f0         ; 29 f0
e315: ORA  $4b          ; 05 4b
e317: STA  $0f2a,X      ; 8d 2a 0f
e31a: LDA  ($1e)        ; b2 1e
e31c: .byte $af   ;?
e31d: .byte $27   ;?
e31e: STY  $e2,X        ; 94 e2
e320: ASL  $10,X        ; 16 10
e322: BPL  $e338        ; 10 14
e324: CMP  ($40)        ; d2 40
e326: ROL               ; 2a 
e327: .byte $0f   ;?
e328: LDA  #$01         ; a9 01
e32a: STA  $4b          ; 85 4b
e32c: LDA  $089e,X      ; ad 9e 08
e32f: AND  #$fc         ; 29 fc
e331: ORA  $4b          ; 05 4b
e333: STA  $089e,X      ; 8d 9e 08
e336: BRA  $e34a        ; 80 12
e338: .byte $c2   ;?
e339: RTI               ; 40 
e33a: ROL               ; 2a 
e33b: .byte $0f   ;?
e33c: LDA  #$00         ; a9 00
e33e: STA  $4b          ; 85 4b
e340: LDA  $089e,X      ; ad 9e 08
e343: AND  #$fc         ; 29 fc
e345: ORA  $4b          ; 05 4b
e347: STA  $089e,X      ; 8d 9e 08
e34a: CMP  ($20)        ; d2 20
e34c: ROL               ; 2a 
e34d: .byte $0f   ;?
e34e: CMP  ($10)        ; d2 10
e350: ROL               ; 2a 
e351: .byte $0f   ;?
e352: CMP  ($02)        ; d2 02
e354: BMI  $e365        ; 30 0f
e356: .byte $c2   ;?
e357: ORA  ($30,X)      ; 01 30
e359: .byte $0f   ;?
e35a: SBC  ($2b)        ; f2 2b
e35c: .byte $0f   ;?
e35d: BPL  $e367        ; 10 08
e35f: .byte $2f   ;?
e360: STY  $05,X        ; 94 05
e362: LDA  ($0a)        ; b2 0a
e364: .byte $af   ;?
e365: .byte $27   ;?
e366: STY  $f2,X        ; 94 f2
e368: ASL  $10,X        ; 16 10
e36a: RTI               ; 40 
e36b: ASL  $c2,X        ; 06 c2
e36d: JSR  $0f2b        ; 20 2b 0f
e370: BRA  $e3a1        ; 80 2f
e372: SBC  ($2b)        ; f2 2b
e374: .byte $0f   ;?
e375: JSR  $d217        ; 20 17 d2
e378: BRA  $e399        ; 80 1f
e37a: .byte $0f   ;?
e37b: LDA  #$01         ; a9 01
e37d: STA  $4b          ; 85 4b
e37f: LDA  $0f2b,X      ; ad 2b 0f
e382: AND  #$f0         ; 29 f0
e384: ORA  $4b          ; 05 4b
e386: STA  $0f2b,X      ; 8d 2b 0f
e389: LDA  ($1e)        ; b2 1e
e38b: .byte $af   ;?
e38c: .byte $27   ;?
e38d: STY  $e2,X        ; 94 e2
e38f: ASL  $10,X        ; 16 10
e391: BPL  $e399        ; 10 06
e393: CMP  ($40)        ; d2 40
e395: .byte $2b   ;?
e396: .byte $0f   ;?
e397: BRA  $e39d        ; 80 04
e399: .byte $c2   ;?
e39a: RTI               ; 40 
e39b: .byte $2b   ;?
e39c: .byte $0f   ;?
e39d: CMP  ($20)        ; d2 20
e39f: .byte $2b   ;?
e3a0: .byte $0f   ;?
e3a1: CMP  ($10)        ; d2 10
e3a3: .byte $2b   ;?
e3a4: .byte $0f   ;?
e3a5: CMP  ($08)        ; d2 08
e3a7: BMI  $e3b8        ; 30 0f
e3a9: .byte $c2   ;?
e3aa: TSB  $30          ; 04 30
e3ac: .byte $0f   ;?
e3ad: RTS               ; 60 
e3ae: RTS               ; 60 
e3af: JSR  $147d        ; 20 7d 14
e3b2: .byte $2f   ;?
e3b3: EOR  $8f5a,X      ; 4d 5a 8f
e3b6: STA  $f257,X      ; 9d 57 f2
e3b9: .byte $c7   ;?
e3ba: TSB  $08          ; 04 08
e3bc: .byte $1b   ;?
e3bd: .byte $c2   ;?
e3be: JSR  $0895        ; 20 95 08
e3c1: LDA  #$00         ; a9 00
e3c3: STA  $4b          ; 85 4b
e3c5: LDA  $0894,X      ; ad 94 08
e3c8: AND  #$fc         ; 29 fc
e3ca: ORA  $4b          ; 05 4b
e3cc: STA  $0894,X      ; 8d 94 08
e3cf: LDA  $0f44,X      ; ad 44 0f
e3d2: AND  #$3f         ; 29 3f
e3d4: STA  $0f44,X      ; 8d 44 0f
e3d7: RTS               ; 60 
e3d8: .byte $e2   ;?
e3d9: ASL  $10,X        ; 16 10
e3db: BPL  $e3e3        ; 10 06
e3dd: .byte $c2   ;?
e3de: JSR  $0895        ; 20 95 08
e3e1: BRA  $e3e7        ; 80 04
e3e3: CMP  ($20)        ; d2 20
e3e5: STA  $08          ; 95 08
e3e7: LDA  #$02         ; a9 02
e3e9: STA  $4b          ; 85 4b
e3eb: LDA  $0894,X      ; ad 94 08
e3ee: AND  #$fc         ; 29 fc
e3f0: ORA  $4b          ; 05 4b
e3f2: STA  $0894,X      ; 8d 94 08
e3f5: SBC  ($44)        ; f2 44
e3f7: .byte $0f   ;?
e3f8: RTI               ; 40 
e3f9: ORA  $d2          ; 15 d2
e3fb: BRA  $e41c        ; 80 1f
e3fd: .byte $0f   ;?
e3fe: CMP  ($80)        ; d2 80
e400: ASL  $b20f        ; 1e 0f b2
e403: ASL               ; 0a 
e404: .byte $af   ;?
e405: .byte $27   ;?
e406: STY  $ad,X        ; 94 ad
e408: TSB  $0f          ; 44 0f
e40a: ORA  #$c0         ; 09 c0
e40c: STA  $0f44,X      ; 8d 44 0f
e40f: RTS               ; 60 
e410: JSR  $147a        ; 20 7a 14
e413: CMP  ($40)        ; d2 40
e415: .byte $37   ;?
e416: .byte $0f   ;?
e417: CMP  ($40)        ; d2 40
e419: .byte $39   ;?
e41a: .byte $0f   ;?
e41b: JSR  $e491        ; 20 91 e4
e41e: .byte $c2   ;?
e41f: .byte $02   ;?
e420: BMI  $e431        ; 30 0f
e422: CMP  ($01)        ; d2 01
e424: BMI  $e435        ; 30 0f
e426: .byte $c2   ;?
e427: PHP               ; 08 
e428: BMI  $e439        ; 30 0f
e42a: CMP  ($04)        ; d2 04
e42c: BMI  $e43d        ; 30 0f
e42e: .byte $c2   ;?
e42f: BPL  $e45b        ; 10 2a
e431: .byte $0f   ;?
e432: .byte $c2   ;?
e433: BPL  $e460        ; 10 2b
e435: .byte $0f   ;?
e436: CMP  ($02)        ; d2 02
e438: JSR  $c20f        ; 20 0f c2
e43b: ORA  ($20,X)      ; 01 20
e43d: .byte $0f   ;?
e43e: SBC  ($16)        ; f2 16
e440: BPL  $e462        ; 10 20
e442: PHP               ; 08 
e443: .byte $c2   ;?
e444: JSR  $0f2a        ; 20 2a 0f
e447: .byte $c2   ;?
e448: JSR  $0f2b        ; 20 2b 0f
e44b: RTS               ; 60 
e44c: LDA  $1017,X      ; ad 17 10
e44f: AND  #$07         ; 29 07
e451: PHA               ; 48 
e452: ASL               ; 0a 
e453: ASL               ; 0a 
e454: ASL               ; 0a 
e455: ASL               ; 0a 
e456: ASL               ; 0a 
e457: STA  $4b          ; 85 4b
e459: LDA  $0840,X      ; ad 40 08
e45c: AND  #$1f         ; 29 1f
e45e: ORA  $4b          ; 05 4b
e460: STA  $0840,X      ; 8d 40 08
e463: PLA               ; 68 
e464: ASL               ; 0a 
e465: ASL               ; 0a 
e466: STA  $4b          ; 85 4b
e468: LDA  $0840,X      ; ad 40 08
e46b: AND  #$e3         ; 29 e3
e46d: ORA  $4b          ; 05 4b
e46f: STA  $0840,X      ; 8d 40 08
e472: LDA  $1017,X      ; ad 17 10
e475: AND  #$38         ; 29 38
e477: LSR               ; 4a 
e478: LSR               ; 4a 
e479: LSR               ; 4a 
e47a: CMP  #$00         ; c9 00
e47c: STA  $4b          ; 85 4b
e47e: LDA  $0847,X      ; ad 47 08
e481: AND  #$f8         ; 29 f8
e483: ORA  $4b          ; 05 4b
e485: STA  $0847,X      ; 8d 47 08
e488: .byte $c2   ;?
e489: .byte $02   ;?
e48a: RTI               ; 40 
e48b: PHP               ; 08 
e48c: CMP  ($01)        ; d2 01
e48e: RTI               ; 40 
e48f: PHP               ; 08 
e490: RTS               ; 60 
e491: .byte $c2   ;?
e492: ORA  ($40,X)      ; 01 40
e494: PHP               ; 08 
e495: .byte $c2   ;?
e496: ORA  ($41,X)      ; 01 41
e498: PHP               ; 08 
e499: .byte $c2   ;?
e49a: .byte $02   ;?
e49b: EOR  ($08,X)      ; 41 08
e49d: CMP  ($02)        ; d2 02
e49f: RTI               ; 40 
e4a0: PHP               ; 08 
e4a1: RTS               ; 60 
e4a2: .byte $5f   ;?
e4a3: .byte $4f   ;?
e4a4: .byte $03   ;?
e4a5: .byte $2f   ;?
e4a6: .byte $a3   ;?
e4a7: JSR  $01d2        ; 20 d2 01
e4aa: TRB  ($d20f)      ; 1c 0f d2
e4ad: BRA  $e4fd        ; 80 4e
e4af: .byte $0f   ;?
e4b0: CMP  ($40)        ; d2 40
e4b2: LSR  $a90f        ; 4e 0f a9
e4b5: LSR  $85,X        ; 46 85
e4b7: LDA  $00a9,X      ; ad a9 00
e4ba: STA  $ae          ; 85 ae
e4bc: .byte $17   ;?
e4bd: STY  $a9,X        ; 94 a9
e4bf: CMP  #$8d         ; c9 8d
e4c1: PHA               ; 48 
e4c2: ORA  ($a9)        ; 12 a9
e4c4: CPX  $8d          ; e4 8d
e4c6: EOR  #$12         ; 49 12
e4c8: RTS               ; 60 
e4c9: .byte $1f   ;?
e4ca: STY  $30,X        ; 94 30
e4cc: CMP  ($40)        ; d2 40
e4ce: .byte $99   ;?
e4cf: PHP               ; 08 
e4d0: CMP  ($08)        ; d2 08
e4d2: STA  $d208,X      ; 9d 08 d2
e4d5: PHP               ; 08 
e4d6: .byte $93   ;?
e4d7: PHP               ; 08 
e4d8: CMP  ($40)        ; d2 40
e4da: ROL  $0f,X        ; 26 0f
e4dc: CMP  ($40)        ; d2 40
e4de: .byte $27   ;?
e4df: .byte $0f   ;?
e4e0: .byte $c2   ;?
e4e1: JSR  $0f2c        ; 20 2c 0f
e4e4: LDA  #$03         ; a9 03
e4e6: STA  $0f18,X      ; 8d 18 0f
e4e9: CMP  ($02)        ; d2 02
e4eb: ROL  $b20f        ; 3e 0f b2
e4ee: TRB  $ad          ; 14 ad
e4f0: .byte $17   ;?
e4f1: STY  $a9,X        ; 94 a9
e4f3: SBC  $488d,X      ; fd 8d 48
e4f6: ORA  ($a9)        ; 12 a9
e4f8: CPX  $8d          ; e4 8d
e4fa: EOR  #$12         ; 49 12
e4fc: RTS               ; 60 
e4fd: .byte $9f   ;?
e4fe: STY  $01,X        ; 94 01
e500: RTS               ; 60 
e501: .byte $c2   ;?
e502: PHP               ; 08 
e503: .byte $93   ;?
e504: PHP               ; 08 
e505: JSR  $dab2        ; 20 b2 da
e508: CMP  ($02)        ; d2 02
e50a: RTI               ; 40 
e50b: .byte $0f   ;?
e50c: .byte $c7   ;?
e50d: ORA  $47          ; 05 47
e50f: ORA  ($c2,X)      ; 01 c2
e511: ORA  ($c0,X)      ; 01 c0
e513: TSB  $af          ; 04 af
e515: EOR  $c20e,X      ; 4d 0e c2
e518: BPL  $e4da        ; 10 c0
e51a: TSB  $d2          ; 04 d2
e51c: ORA  ($c5,X)      ; 01 c5
e51e: TSB  $c2          ; 04 c2
e520: JSR  $04c0        ; 20 c0 04
e523: BRA  $e540        ; 80 1b
e525: .byte $d7   ;?
e526: STZ  ($10d2)      ; 9c d2 10
e529: CPY  #$04         ; c0 04
e52b: .byte $3f   ;?
e52c: EOR  $d20a,X      ; 4d 0a d2
e52f: ORA  ($c5,X)      ; 01 c5
e531: TSB  $d2          ; 04 d2
e533: JSR  $04c0        ; 20 c0 04
e536: BRA  $e540        ; 80 08
e538: .byte $c2   ;?
e539: ORA  ($c5,X)      ; 01 c5
e53b: TSB  $d2          ; 04 d2
e53d: JSR  $04c0        ; 20 c0 04
e540: .byte $af   ;?
e541: EOR  $e224,X      ; 4d 24 e2
e544: ASL  $10,X        ; 16 10
e546: JSR  $ad1f        ; 20 1f ad
e549: ASL  $10,X        ; 16 10
e54b: AND  #$0f         ; 29 0f
e54d: ORA  #$20         ; 09 20
e54f: .byte $e2   ;?
e550: ASL  $10,X        ; 16 10
e552: BPL  $e556        ; 10 02
e554: ORA  #$40         ; 09 40
e556: .byte $af   ;?
e557: .byte $9b   ;?
e558: .byte $03   ;?
e559: STA  $0f2a,X      ; 8d 2a 0f
e55c: .byte $9f   ;?
e55d: .byte $9b   ;?
e55e: .byte $03   ;?
e55f: STA  $0f2b,X      ; 8d 2b 0f
e562: LDA  ($28)        ; b2 28
e564: .byte $af   ;?
e565: .byte $27   ;?
e566: STY  $c2,X        ; 94 c2
e568: JSR  $0f2c        ; 20 2c 0f
e56b: CMP  ($80)        ; d2 80
e56d: ASL  $0f,X        ; 16 0f
e56f: CMP  ($80)        ; d2 80
e571: ORA  $0f          ; 15 0f
e573: .byte $67   ;?
e574: .byte $93   ;?
e575: LDA  ($32)        ; b2 32
e577: LDA  $9417,X      ; ad 17 94
e57a: RTS               ; 60 
e57b: JSR  $1408        ; 20 08 14
e57e: .byte $c2   ;?
e57f: ORA  ($1c,X)      ; 01 1c
e581: .byte $0f   ;?
e582: LDA  #$00         ; a9 00
e584: STA  $04f4,X      ; 8d f4 04
e587: .byte $e2   ;?
e588: LSR  $800f        ; 4e 0f 80
e58b: PHA               ; 48 
e58c: .byte $c2   ;?
e58d: .byte $03   ;?
e58e: ROL               ; 2a 
e58f: .byte $0f   ;?
e590: .byte $c2   ;?
e591: .byte $03   ;?
e592: .byte $2b   ;?
e593: .byte $0f   ;?
e594: LDA  $0f44,X      ; ad 44 0f
e597: AND  #$3f         ; 29 3f
e599: STA  $0f44,X      ; 8d 44 0f
e59c: .byte $c2   ;?
e59d: ORA  ($20,X)      ; 01 20
e59f: .byte $0f   ;?
e5a0: .byte $c2   ;?
e5a1: .byte $02   ;?
e5a2: RTI               ; 40 
e5a3: .byte $0f   ;?
e5a4: .byte $c2   ;?
e5a5: .byte $02   ;?
e5a6: ROL  $b20f        ; 3e 0f b2
e5a9: BVC  $e56c        ; 50 c1
e5ab: .byte $07   ;?
e5ac: STX  $20,Y        ; 96 20
e5ae: ROL  $a9,X        ; 36 a9
e5b0: .byte $0f   ;?
e5b1: STX  $fa,Y        ; 96 fa
e5b3: JSR  $e18d        ; 20 8d e1
e5b6: .byte $c2   ;?
e5b7: RTI               ; 40 
e5b8: .byte $99   ;?
e5b9: PHP               ; 08 
e5ba: .byte $c2   ;?
e5bb: PHP               ; 08 
e5bc: STA  $c208,X      ; 9d 08 c2
e5bf: RTI               ; 40 
e5c0: LSR  $c20f        ; 4e 0f c2
e5c3: BRA  $e613        ; 80 4e
e5c5: .byte $0f   ;?
e5c6: .byte $67   ;?
e5c7: .byte $93   ;?
e5c8: LDA  #$a2         ; a9 a2
e5ca: STA  $1248,X      ; 8d 48 12
e5cd: LDA  #$e4         ; a9 e4
e5cf: STA  $1249,X      ; 8d 49 12
e5d2: .byte $e7   ;?
e5d3: .byte $93   ;?
e5d4: RTS               ; 60 
e5d5: .byte $8f   ;?
e5d6: EOR  $207b,X      ; 4d 7b 20
e5d9: .byte $cf   ;?
e5da: .byte $13   ;?
e5db: .byte $2f   ;?
e5dc: EOR  $e23c,X      ; 4d 3c e2
e5df: .byte $c7   ;?
e5e0: TSB  $08          ; 04 08
e5e2: .byte $37   ;?
e5e3: .byte $e2   ;?
e5e4: CPY  #$04         ; c0 04
e5e6: JSR  $e232        ; 20 32 e2
e5e9: .byte $c7   ;?
e5ea: TSB  $20          ; 04 20
e5ec: AND  $80d2,X      ; 2d d2 80
e5ef: ASL  $d20f        ; 1e 0f d2
e5f2: BRA  $e613        ; 80 1f
e5f4: .byte $0f   ;?
e5f5: LDA  ($32)        ; b2 32
e5f7: .byte $af   ;?
e5f8: .byte $27   ;?
e5f9: STY  $ad,X        ; 94 ad
e5fb: CLD               ; d8 
e5fc: TSB  $8d          ; 04 8d
e5fe: CLD               ; d8 
e5ff: TSB  $4a          ; 04 4a
e601: LSR               ; 4a 
e602: LSR               ; 4a 
e603: BCC  $e608        ; 90 03
e605: LDA  #$07         ; a9 07
e607: RTS               ; 60 
e608: LSR               ; 4a 
e609: BCC  $e60e        ; 90 03
e60b: LDA  #$05         ; a9 05
e60d: RTS               ; 60 
e60e: LSR               ; 4a 
e60f: BCC  $e614        ; 90 03
e611: LDA  #$06         ; a9 06
e613: RTS               ; 60 
e614: LSR               ; 4a 
e615: BCC  $e61a        ; 90 03
e617: LDA  #$08         ; a9 08
e619: RTS               ; 60 
e61a: LDA  #$ff         ; a9 ff
e61c: STA  $04d8,X      ; 8d d8 04
e61f: .byte $57   ;?
e620: BRK               ; 00 
e621: .byte $bf   ;?
e622: EOR  $a70b,X      ; 4d 0b a7
e625: ORA  ($20,X)      ; 01 20
e627: ROR  $a5e6        ; 6e e6 a5
e62a: .byte $02   ;?
e62b: .byte $27   ;?
e62c: ORA  ($80,X)      ; 01 80
e62e: .byte $07   ;?
e62f: LDA  $0f7a,X      ; ad 7a 0f
e632: LSR               ; 4a 
e633: LSR               ; 4a 
e634: LSR               ; 4a 
e635: LSR               ; 4a 
e636: .byte $d7   ;?
e637: BRK               ; 00 
e638: AND  #$0f         ; 29 0f
e63a: CMP  #$0d         ; c9 0d
e63c: BCS  $e653        ; b0 15
e63e: TAX               ; aa 
e63f: LDA  $f799,X      ; bd 99 f7
e642: STA  $45          ; 85 45
e644: .byte $d7   ;?
e645: BRK               ; 00 
e646: JSR  $e66e        ; 20 6e e6
e649: LDA  $02          ; a5 02
e64b: .byte $57   ;?
e64c: BRK               ; 00 
e64d: AND  #$0f         ; 29 0f
e64f: CMP  #$0d         ; c9 0d
e651: BCC  $e656        ; 90 03
e653: JMP  ($e66b)      ; 4c 6b e6
e656: TAX               ; aa 
e657: LDA  $f78c,X      ; bd 8c f7
e65a: STA  $44          ; 85 44
e65c: LDX  $44,X        ; a6 44
e65e: BEQ  $e664        ; f0 04
e660: LDA  $45          ; a5 45
e662: BRA  $e667        ; 80 03
e664: LDA  $45          ; a5 45
e666: RTS               ; 60 
e667: BNE  $e66b        ; d0 02
e669: TXA               ; 8a 
e66a: RTS               ; 60 
e66b: LDA  #$00         ; a9 00
e66d: RTS               ; 60 
e66e: JSR  $e673        ; 20 73 e6
e671: BRA  $e673        ; 80 00
e673: .byte $22   ;?
e674: AND  ($22)        ; 32 22
e676: AND  ($22)        ; 32 22
e678: AND  ($ea)        ; 32 ea
e67a: NOP               ; ea 
e67b: NOP               ; ea 
e67c: RTS               ; 60 
e67d: .byte $6f   ;?
e67e: STY  $18,X        ; 94 18
e680: .byte $2f   ;?
e681: .byte $a3   ;?
e682: ORA  $f2          ; 05 f2
e684: LSR  $010e        ; 4e 0e 01
e687: BPL  $e6a8        ; 10 1f
e689: STA  $170d,X      ; 9d 0d 17
e68c: STA  $d520,X      ; 9d 20 d5
e68f: SBC  $cd          ; e5 cd
e691: .byte $67   ;?
e692: ORA  ($f0)        ; 12 f0
e694: TSB  $8d          ; 04 8d
e696: .byte $67   ;?
e697: ORA  ($60)        ; 12 60
e699: .byte $4f   ;?
e69a: .byte $9b   ;?
e69b: ASL  $00c9        ; 0e c9 00
e69e: BEQ  $e6a1        ; f0 01
e6a0: RTS               ; 60 
e6a1: .byte $47   ;?
e6a2: .byte $9b   ;?
e6a3: LDA  ($0a)        ; b2 0a
e6a5: CMP  #$67         ; c9 67
e6a7: STY  $a9,X        ; 94 a9
e6a9: BRK               ; 00 
e6aa: CMP  $1009,X      ; cd 09 10
e6ad: BEQ  $e698        ; f0 e9
e6af: STA  $1009,X      ; 8d 09 10
e6b2: CMP  #$00         ; c9 00
e6b4: BEQ  $e6ba        ; f0 04
e6b6: LDY  #$00         ; a0 00
e6b8: STY  $d1          ; 84 d1
e6ba: LDX  #$0f         ; a2 0f
e6bc: LDA  $50          ; b5 50
e6be: CMP  $1009,X      ; cd 09 10
e6c1: BEQ  $e6c8        ; f0 05
e6c3: DEX               ; ca 
e6c4: BPL  $e6bc        ; 10 f6
e6c6: LDX  #$00         ; a2 00
e6c8: .byte $07   ;?
e6c9: .byte $9b   ;?
e6ca: .byte $97   ;?
e6cb: .byte $93   ;?
e6cc: TXA               ; 8a 
e6cd: ASL               ; 0a 
e6ce: TAX               ; aa 
e6cf: JSR  $13d2        ; 20 d2 13
e6d2: JMP  ($f7a6,X)    ; 7c a6 f7
e6d5: JSR  $13d5        ; 20 d5 13
e6d8: .byte $2f   ;?
e6d9: .byte $a3   ;?
e6da: AND  $8f          ; 25 8f
e6dc: .byte $4f   ;?
e6dd: ORA  $9f          ; 05 9f
e6df: .byte $4f   ;?
e6e0: .byte $02   ;?
e6e1: BRA  $e6e5        ; 80 02
e6e3: AND  #$fc         ; 29 fc
e6e5: CMP  $127d,X      ; cd 7d 12
e6e8: BEQ  $e700        ; f0 16
e6ea: STA  $127d,X      ; 8d 7d 12
e6ed: STA  $138d,X      ; 8d 8d 13
e6f0: LDA  #$01         ; a9 01
e6f2: STA  $127c,X      ; 8d 7c 12
e6f5: STA  $138c,X      ; 8d 8c 13
e6f8: LDA  #$02         ; a9 02
e6fa: STA  $138b,X      ; 8d 8b 13
e6fd: JSR  $c9c2        ; 20 c2 c9
e700: RTS               ; 60 
e701: JSR  $13d8        ; 20 d8 13
e704: .byte $2f   ;?
e705: .byte $a3   ;?
e706: TRB  ($02a9)      ; 1c a9 02
e709: STA  $127e,X      ; 8d 7e 12
e70c: STA  $138c,X      ; 8d 8c 13
e70f: LDA  $127f,X      ; ad 7f 12
e712: STA  $138d,X      ; 8d 8d 13
e715: LDA  $1280,X      ; ad 80 12
e718: STA  $138e,X      ; 8d 8e 13
e71b: LDA  #$03         ; a9 03
e71d: STA  $138b,X      ; 8d 8b 13
e720: JSR  $c9c2        ; 20 c2 c9
e723: RTS               ; 60 
e724: LDY  ($12c7)      ; ac c7 12
e727: JSR  $13db        ; 20 db 13
e72a: .byte $2f   ;?
e72b: .byte $a3   ;?
e72c: ROL  $4ef2        ; 2e f2 4e
e72f: ASL  $2901        ; 0e 01 29
e732: .byte $7f   ;?
e733: .byte $4f   ;?
e734: ROL  $cd,X        ; 26 cd
e736: DEC  $12,X        ; c6 12
e738: BNE  $e745        ; d0 0b
e73a: CPY  ($12c7)      ; cc c7 12
e73d: BEQ  $e75b        ; f0 1c
e73f: STY  ($12c7)      ; 8c c7 12
e742: STY  ($138e)      ; 8c 8e 13
e745: STA  $12c6,X      ; 8d c6 12
e748: STA  $138d,X      ; 8d 8d 13
e74b: LDA  #$08         ; a9 08
e74d: STA  $127c,X      ; 8d 7c 12
e750: STA  $138c,X      ; 8d 8c 13
e753: LDA  #$08         ; a9 08
e755: STA  $138b,X      ; 8d 8b 13
e758: JSR  $c9c2        ; 20 c2 c9
e75b: RTS               ; 60 
e75c: .byte $8f   ;?
e75d: STY  $01,X        ; 94 01
e75f: RTS               ; 60 
e760: LDA  $1009,X      ; ad 09 10
e763: JSR  $13fc        ; 20 fc 13
e766: .byte $9f   ;?
e767: .byte $4f   ;?
e768: .byte $03   ;?
e769: .byte $af   ;?
e76a: .byte $a3   ;?
e76b: AND  $4f8f,X      ; 3d 8f 4f
e76e: .byte $3f   ;?
e76f: CMP  $56          ; c5 56
e771: BEQ  $e791        ; f0 1e
e773: CMP  $55          ; c5 55
e775: BNE  $e7a9        ; d0 32
e777: LDA  $100a,X      ; ad 0a 10
e77a: BMI  $e780        ; 30 04
e77c: CMP  #$05         ; c9 05
e77e: BCS  $e783        ; b0 03
e780: INC  $100a        ; ee 0a 10
e783: LDA  $100b,X      ; ad 0b 10
e786: BMI  $e78c        ; 30 04
e788: CMP  #$05         ; c9 05
e78a: BCS  $e7a9        ; b0 1d
e78c: INC  $100b        ; ee 0b 10
e78f: BRA  $e7a9        ; 80 18
e791: LDA  $100a,X      ; ad 0a 10
e794: BPL  $e79a        ; 10 04
e796: CMP  #$b7         ; c9 b7
e798: BCC  $e79d        ; 90 03
e79a: DEC  $100a        ; ce 0a 10
e79d: LDA  $100b,X      ; ad 0b 10
e7a0: BPL  $e7a6        ; 10 04
e7a2: CMP  #$b7         ; c9 b7
e7a4: BCC  $e7a9        ; 90 03
e7a6: DEC  $100b        ; ce 0b 10
e7a9: LDA  ($05)        ; b2 05
e7ab: .byte $ab   ;?
e7ac: .byte $07   ;?
e7ad: STY  $60,X        ; 94 60
e7af: JSR  $1405        ; 20 05 14
e7b2: .byte $2f   ;?
e7b3: EOR  $3f30,X      ; 4d 30 3f
e7b6: EOR  $ff2d,X      ; 4d 2d ff
e7b9: .byte $02   ;?
e7ba: .byte $07   ;?
e7bb: .byte $6f   ;?
e7bc: .byte $9b   ;?
e7bd: ORA  ($67)        ; 12 67
e7bf: .byte $9b   ;?
e7c0: BRA  $e7c7        ; 80 05
e7c2: .byte $ef   ;?
e7c3: .byte $9b   ;?
e7c4: .byte $0b   ;?
e7c5: .byte $e7   ;?
e7c6: .byte $9b   ;?
e7c7: .byte $57   ;?
e7c8: .byte $9b   ;?
e7c9: LDA  ($05)        ; b2 05
e7cb: .byte $b3   ;?
e7cc: .byte $47   ;?
e7cd: STY  $80,X        ; 94 80
e7cf: ORA  $4f          ; 15 4f
e7d1: STY  $12,X        ; 94 12
e7d3: .byte $df   ;?
e7d4: .byte $9b   ;?
e7d5: .byte $0f   ;?
e7d6: .byte $d7   ;?
e7d7: .byte $9b   ;?
e7d8: .byte $6f   ;?
e7d9: .byte $9b   ;?
e7da: ASL  $d2,X        ; 06 d2
e7dc: BPL  $e7a3        ; 10 c5
e7de: TSB  $80          ; 04 80
e7e0: TSB  $c2          ; 04 c2
e7e2: BPL  $e7a9        ; 10 c5
e7e4: TSB  $3f          ; 04 3f
e7e6: EOR  $a904,X      ; 4d 04 a9
e7e9: BMI  $e76b        ; 30 80
e7eb: .byte $03   ;?
e7ec: LDA  $0f7a,X      ; ad 7a 0f
e7ef: AND  #$f0         ; 29 f0
e7f1: STA  $40          ; 85 40
e7f3: LDA  $04c7,X      ; ad c7 04
e7f6: AND  #$08         ; 29 08
e7f8: ORA  $40          ; 05 40
e7fa: STA  $40          ; 85 40
e7fc: LDA  $9a          ; a5 9a
e7fe: AND  #$f8         ; 29 f8
e800: EOR  $40          ; 45 40
e802: BNE  $e805        ; d0 01
e804: RTS               ; 60 
e805: PHA               ; 48 
e806: .byte $2f   ;?
e807: .byte $a3   ;?
e808: .byte $19   ;?
e809: .byte $e2   ;?
e80a: LSR  $010e        ; 4e 0e 01
e80d: .byte $02   ;?
e80e: PLA               ; 68 
e80f: RTS               ; 60 
e810: LDA  $40          ; a5 40
e812: .byte $3f   ;?
e813: EOR  $2902,X      ; 4d 02 29
e816: PHP               ; 08 
e817: CMP  $1280,X      ; cd 80 12
e81a: BEQ  $e822        ; f0 06
e81c: STA  $1280,X      ; 8d 80 12
e81f: JSR  $e701        ; 20 01 e7
e822: PLA               ; 68 
e823: PHA               ; 48 
e824: .byte $5f   ;?
e825: STZ  ($a516)      ; 9c 16 a5
e828: RTI               ; 40 
e829: CMP  #$f8         ; c9 f8
e82b: BEQ  $e83d        ; f0 10
e82d: AND  #$18         ; 29 18
e82f: CMP  #$18         ; c9 18
e831: BNE  $e83d        ; d0 0a
e833: CMP  ($20)        ; d2 20
e835: PHX               ; da 
e836: TSB  $c2          ; 04 c2
e838: JSR  $04da        ; 20 da 04
e83b: .byte $57   ;?
e83c: STZ  ($8568)      ; 9c 68 85
e83f: RTI               ; 40 
e840: ASL  $40,X        ; 06 40
e842: BCC  $e84f        ; 90 0b
e844: SBC  ($7a)        ; f2 7a
e846: .byte $0f   ;?
e847: BRA  $e84d        ; 80 04
e849: .byte $77   ;?
e84a: TXS               ; 9a 
e84b: BRA  $e84f        ; 80 02
e84d: .byte $f7   ;?
e84e: TXS               ; 9a 
e84f: ASL  $40,X        ; 06 40
e851: BCC  $e85e        ; 90 0b
e853: SBC  ($7a)        ; f2 7a
e855: .byte $0f   ;?
e856: RTI               ; 40 
e857: TSB  $67          ; 04 67
e859: TXS               ; 9a 
e85a: BRA  $e85e        ; 80 02
e85c: .byte $e7   ;?
e85d: TXS               ; 9a 
e85e: ASL  $40,X        ; 06 40
e860: BCC  $e86d        ; 90 0b
e862: SBC  ($7a)        ; f2 7a
e864: .byte $0f   ;?
e865: JSR  $5704        ; 20 04 57
e868: TXS               ; 9a 
e869: BRA  $e86d        ; 80 02
e86b: .byte $d7   ;?
e86c: TXS               ; 9a 
e86d: ASL  $40,X        ; 06 40
e86f: BCC  $e892        ; 90 21
e871: SBC  ($7a)        ; f2 7a
e873: .byte $0f   ;?
e874: BPL  $e87a        ; 10 04
e876: .byte $47   ;?
e877: TXS               ; 9a 
e878: BRA  $e892        ; 80 18
e87a: .byte $c7   ;?
e87b: TXS               ; 9a 
e87c: .byte $e2   ;?
e87d: .byte $13   ;?
e87e: BPL  $e800        ; 10 80
e880: ORA  ($4f),Y      ; 11 4f
e882: EOR  $870e,X      ; 4d 0e 87
e885: STA  $80d2,X      ; 9d d2 80
e888: TSB  $0f          ; 44 0f
e88a: CMP  ($40)        ; d2 40
e88c: TSB  $0f          ; 44 0f
e88e: CMP  ($08)        ; d2 08
e890: INA               ; 1a 
e891: .byte $0f   ;?
e892: ASL  $40,X        ; 06 40
e894: BCC  $e8a1        ; 90 0b
e896: SBC  ($c7)        ; f2 c7
e898: TSB  $08          ; 04 08
e89a: TSB  $37          ; 04 37
e89c: TXS               ; 9a 
e89d: BRA  $e8a1        ; 80 02
e89f: .byte $b7   ;?
e8a0: TXS               ; 9a 
e8a1: LDA  ($02)        ; b2 02
e8a3: .byte $c7   ;?
e8a4: .byte $57   ;?
e8a5: STY  $60,X        ; 94 60
e8a7: RTS               ; 60 
e8a8: LDA  #$a7         ; a9 a7
e8aa: STA  $12cd,X      ; 8d cd 12
e8ad: LDA  #$e8         ; a9 e8
e8af: STA  $12ce,X      ; 8d ce 12
e8b2: RTS               ; 60 
e8b3: LDA  #$d5         ; a9 d5
e8b5: STA  $12cd,X      ; 8d cd 12
e8b8: LDA  #$e6         ; a9 e6
e8ba: STA  $12ce,X      ; 8d ce 12
e8bd: RTS               ; 60 
e8be: LDA  #$24         ; a9 24
e8c0: STA  $12cd,X      ; 8d cd 12
e8c3: LDA  #$e7         ; a9 e7
e8c5: STA  $12ce,X      ; 8d ce 12
e8c8: RTS               ; 60 
e8c9: LDA  #$b0         ; a9 b0
e8cb: STA  $12cd,X      ; 8d cd 12
e8ce: LDA  #$e9         ; a9 e9
e8d0: STA  $12ce,X      ; 8d ce 12
e8d3: RTS               ; 60 
e8d4: .byte $17   ;?
e8d5: .byte $93   ;?
e8d6: LDA  #$00         ; a9 00
e8d8: JMP  ($12cd)      ; 6c cd 12
e8db: JSR  $e8a8        ; 20 a8 e8
e8de: RTS               ; 60 
e8df: JSR  $e8a8        ; 20 a8 e8
e8e2: RTS               ; 60 
e8e3: JSR  $e8a8        ; 20 a8 e8
e8e6: RTS               ; 60 
e8e7: RTS               ; 60 
e8e8: JSR  $e8a8        ; 20 a8 e8
e8eb: LDA  $11c8,X      ; ad c8 11
e8ee: EOR  #$02         ; 49 02
e8f0: ORA  #$01         ; 09 01
e8f2: LDA  $11c8,X      ; ad c8 11
e8f5: RTS               ; 60 
e8f6: RTS               ; 60 
e8f7: JSR  $e8b3        ; 20 b3 e8
e8fa: .byte $af   ;?
e8fb: .byte $a3   ;?
e8fc: TRB  ($12e2)      ; 1c e2 12
e8ff: BPL  $e903        ; 10 02
e901: .byte $0f   ;?
e902: .byte $e2   ;?
e903: ORA  ($10)        ; 12 10
e905: ORA  ($0a,X)      ; 01 0a
e907: .byte $c2   ;?
e908: .byte $02   ;?
e909: ORA  ($10)        ; 12 10
e90b: .byte $c2   ;?
e90c: ORA  ($12,X)      ; 01 12
e90e: BPL  $e890        ; 10 80
e910: PHP               ; 08 
e911: CMP  ($02)        ; d2 02
e913: ORA  ($10)        ; 12 10
e915: CMP  ($01)        ; d2 01
e917: ORA  ($10)        ; 12 10
e919: LDA  $127d,X      ; ad 7d 12
e91c: ORA  #$04         ; 09 04
e91e: JMP  ($e6d5)      ; 4c d5 e6
e921: JSR  $e8b3        ; 20 b3 e8
e924: LDA  $127d,X      ; ad 7d 12
e927: ORA  #$01         ; 09 01
e929: JMP  ($e6d5)      ; 4c d5 e6
e92c: JSR  $e8b3        ; 20 b3 e8
e92f: LDA  $127d,X      ; ad 7d 12
e932: ORA  #$02         ; 09 02
e934: JMP  ($e6d5)      ; 4c d5 e6
e937: JSR  $e8a8        ; 20 a8 e8
e93a: .byte $e2   ;?
e93b: ORA  ($10)        ; 12 10
e93d: PHP               ; 08 
e93e: TRB  $e2          ; 14 e2
e940: ORA  ($10)        ; 12 10
e942: TSB  $0f          ; 04 0f
e944: .byte $c2   ;?
e945: PHP               ; 08 
e946: ORA  ($10)        ; 12 10
e948: .byte $c2   ;?
e949: TSB  $12          ; 04 12
e94b: BPL  $e8fa        ; 10 ad
e94d: DEC  $12,X        ; c6 12
e94f: AND  #$fe         ; 29 fe
e951: BRA  $e960        ; 80 0d
e953: CMP  ($08)        ; d2 08
e955: ORA  ($10)        ; 12 10
e957: CMP  ($04)        ; d2 04
e959: ORA  ($10)        ; 12 10
e95b: LDA  $12c6,X      ; ad c6 12
e95e: ORA  #$01         ; 09 01
e960: JMP  ($e724)      ; 4c 24 e7
e963: .byte $7f   ;?
e964: .byte $4f   ;?
e965: ASL               ; 0a 
e966: LDA  #$71         ; a9 71
e968: STA  $12cd,X      ; 8d cd 12
e96b: LDA  #$e9         ; a9 e9
e96d: STA  $12ce,X      ; 8d ce 12
e970: RTS               ; 60 
e971: LDA  $12c6,X      ; ad c6 12
e974: ORA  #$02         ; 09 02
e976: JMP  ($e724)      ; 4c 24 e7
e979: .byte $7f   ;?
e97a: .byte $4f   ;?
e97b: ORA  $a820,X      ; 0d 20 a8
e97e: INX               ; e8 
e97f: .byte $17   ;?
e980: STZ  $c6ad,X      ; 9e ad c6
e983: ORA  ($29)        ; 12 29
e985: SBC  $2420,X      ; fd 20 24
e988: .byte $e7   ;?
e989: RTS               ; 60 
e98a: NOP               ; ea 
e98b: NOP               ; ea 
e98c: NOP               ; ea 
e98d: NOP               ; ea 
e98e: NOP               ; ea 
e98f: BRA  $e991        ; 80 00
e991: .byte $c7   ;?
e992: .byte $9b   ;?
e993: JMP  ($e8a8)      ; 4c a8 e8
e996: LDA  #$a1         ; a9 a1
e998: STA  $12cd,X      ; 8d cd 12
e99b: LDA  #$e9         ; a9 e9
e99d: STA  $12ce,X      ; 8d ce 12
e9a0: RTS               ; 60 
e9a1: JSR  $e8a8        ; 20 a8 e8
e9a4: JSR  $a4d0        ; 20 d0 a4
e9a7: .byte $e2   ;?
e9a8: .byte $c7   ;?
e9a9: TSB  $08          ; 04 08
e9ab: .byte $03   ;?
e9ac: JMP  ($e937)      ; 4c 37 e9
e9af: RTS               ; 60 
e9b0: TAY               ; a8 
e9b1: JSR  $e8c9        ; 20 c9 e8
e9b4: LDA  $12c6,X      ; ad c6 12
e9b7: JMP  ($e727)      ; 4c 27 e7
e9ba: .byte $0f   ;?
e9bb: .byte $9b   ;?
e9bc: .byte $02   ;?
e9bd: BRA  $e9c2        ; 80 03
e9bf: LDA  $1009,X      ; ad 09 10
e9c2: RTS               ; 60 
e9c3: JSR  $12d3        ; 20 d3 12
e9c6: LDA  $0f7f,X      ; ad 7f 0f
e9c9: AND  #$70         ; 29 70
e9cb: LSR               ; 4a 
e9cc: LSR               ; 4a 
e9cd: LSR               ; 4a 
e9ce: LSR               ; 4a 
e9cf: CMP  #$00         ; c9 00
e9d1: ASL               ; 0a 
e9d2: TAX               ; aa 
e9d3: JMP  ($f7c6,X)    ; 7c c6 f7
e9d6: LDA  #$01         ; a9 01
e9d8: STA  $1005,X      ; 8d 05 10
e9db: .byte $47   ;?
e9dc: .byte $4f   ;?
e9dd: .byte $c2   ;?
e9de: BRA  $ea06        ; 80 26
e9e0: .byte $0f   ;?
e9e1: .byte $c2   ;?
e9e2: BRA  $ea0b        ; 80 27
e9e4: .byte $0f   ;?
e9e5: .byte $c2   ;?
e9e6: BRA  $e9ff        ; 80 17
e9e8: BPL  $e9bc        ; 10 d2
e9ea: .byte $02   ;?
e9eb: RTI               ; 40 
e9ec: PHP               ; 08 
e9ed: LDA  #$06         ; a9 06
e9ef: STA  $1268,X      ; 8d 68 12
e9f2: LDA  #$00         ; a9 00
e9f4: STA  $0f00,X      ; 8d 00 0f
e9f7: LDA  #$03         ; a9 03
e9f9: STA  $1269,X      ; 8d 69 12
e9fc: .byte $c2   ;?
e9fd: BPL  $ea40        ; 10 41
e9ff: PHP               ; 08 
ea00: .byte $c2   ;?
ea01: PHP               ; 08 
ea02: INY               ; c8 
ea03: ORA  ($c2),Y      ; 11 c2
ea05: ORA  ($65,X)      ; 01 65
ea07: .byte $0f   ;?
ea08: .byte $47   ;?
ea09: EOR  $4d87,X      ; 4d 87 4d
ea0c: .byte $27   ;?
ea0d: EOR  $10c2,X      ; 4d c2 10
ea10: CPY  #$04         ; c0 04
ea12: RTS               ; 60 
ea13: LDA  #$02         ; a9 02
ea15: STA  $1005,X      ; 8d 05 10
ea18: .byte $47   ;?
ea19: .byte $4f   ;?
ea1a: .byte $c2   ;?
ea1b: BRA  $ea43        ; 80 26
ea1d: .byte $0f   ;?
ea1e: .byte $c2   ;?
ea1f: BRA  $ea48        ; 80 27
ea21: .byte $0f   ;?
ea22: .byte $c2   ;?
ea23: BRA  $ea3c        ; 80 17
ea25: BPL  $e9f9        ; 10 d2
ea27: .byte $02   ;?
ea28: RTI               ; 40 
ea29: PHP               ; 08 
ea2a: LDA  #$06         ; a9 06
ea2c: STA  $1268,X      ; 8d 68 12
ea2f: LDA  #$00         ; a9 00
ea31: STA  $0f00,X      ; 8d 00 0f
ea34: LDA  #$03         ; a9 03
ea36: STA  $1269,X      ; 8d 69 12
ea39: .byte $c2   ;?
ea3a: BPL  $ea7d        ; 10 41
ea3c: PHP               ; 08 
ea3d: .byte $c2   ;?
ea3e: PHP               ; 08 
ea3f: INY               ; c8 
ea40: ORA  ($c2),Y      ; 11 c2
ea42: ORA  ($65,X)      ; 01 65
ea44: .byte $0f   ;?
ea45: .byte $47   ;?
ea46: EOR  $a960,X      ; 4d 60 a9
ea49: .byte $03   ;?
ea4a: STA  $1005,X      ; 8d 05 10
ea4d: .byte $47   ;?
ea4e: .byte $4f   ;?
ea4f: .byte $c2   ;?
ea50: BRA  $ea78        ; 80 26
ea52: .byte $0f   ;?
ea53: .byte $c2   ;?
ea54: BRA  $ea7d        ; 80 27
ea56: .byte $0f   ;?
ea57: .byte $c2   ;?
ea58: BRA  $ea71        ; 80 17
ea5a: BPL  $ea2e        ; 10 d2
ea5c: .byte $02   ;?
ea5d: RTI               ; 40 
ea5e: PHP               ; 08 
ea5f: RTS               ; 60 
ea60: LDA  #$04         ; a9 04
ea62: STA  $1005,X      ; 8d 05 10
ea65: .byte $47   ;?
ea66: .byte $4f   ;?
ea67: .byte $c2   ;?
ea68: BRA  $ea90        ; 80 26
ea6a: .byte $0f   ;?
ea6b: .byte $c2   ;?
ea6c: BRA  $ea95        ; 80 27
ea6e: .byte $0f   ;?
ea6f: .byte $47   ;?
ea70: EOR  $a960,X      ; 4d 60 a9
ea73: ORA  $8d          ; 05 8d
ea75: ORA  $10          ; 05 10
ea77: RTS               ; 60 
ea78: LDA  #$06         ; a9 06
ea7a: STA  $1005,X      ; 8d 05 10
ea7d: RTS               ; 60 
ea7e: LDA  #$08         ; a9 08
ea80: STA  $1005,X      ; 8d 05 10
ea83: .byte $47   ;?
ea84: .byte $4f   ;?
ea85: .byte $c2   ;?
ea86: BRA  $eaae        ; 80 26
ea88: .byte $0f   ;?
ea89: .byte $c2   ;?
ea8a: BRA  $eab3        ; 80 27
ea8c: .byte $0f   ;?
ea8d: .byte $c2   ;?
ea8e: BRA  $eaa6        ; 80 16
ea90: BPL  $eab2        ; 10 20
ea92: AND  ($e2,X)      ; 21 e2
ea94: RTS               ; 60 
ea95: JSR  $13cc        ; 20 cc 13
ea98: JSR  $eab8        ; 20 b8 ea
ea9b: JSR  $eaa1        ; 20 a1 ea
ea9e: JMP  ($ead5)      ; 4c d5 ea
eaa1: .byte $1f   ;?
eaa2: STZ  $d70a,X      ; 9e 0a d7
eaa5: ORA  ($ad,X)      ; 01 ad
eaa7: DEC  $12,X        ; c6 12
eaa9: ORA  #$02         ; 09 02
eaab: JMP  ($e724)      ; 4c 24 e7
eaae: .byte $57   ;?
eaaf: ORA  ($ad,X)      ; 01 ad
eab1: DEC  $12,X        ; c6 12
eab3: AND  #$fd         ; 29 fd
eab5: JMP  ($e724)      ; 4c 24 e7
eab8: .byte $e2   ;?
eab9: ORA  ($10)        ; 12 10
eabb: PHP               ; 08 
eabc: ASL  $12e2        ; 0e e2 12
eabf: BPL  $eac5        ; 10 04
eac1: ORA  #$b7         ; 09 b7
eac3: ORA  ($ad,X)      ; 01 ad
eac5: DEC  $12,X        ; c6 12
eac7: ORA  #$01         ; 09 01
eac9: BRA  $ead2        ; 80 07
eacb: .byte $37   ;?
eacc: ORA  ($ad,X)      ; 01 ad
eace: DEC  $12,X        ; c6 12
ead0: AND  #$fe         ; 29 fe
ead2: JMP  ($e724)      ; 4c 24 e7
ead5: .byte $e2   ;?
ead6: ORA  ($10)        ; 12 10
ead8: .byte $02   ;?
ead9: PHP               ; 08 
eada: .byte $e2   ;?
eadb: ORA  ($10)        ; 12 10
eadd: ORA  ($03,X)      ; 01 03
eadf: .byte $87   ;?
eae0: ORA  ($60,X)      ; 01 60
eae2: .byte $07   ;?
eae3: ORA  ($60,X)      ; 01 60
eae5: LDX  #$00         ; a2 00
eae7: LDA  #$20         ; a9 20
eae9: STA  $04e6,X      ; 8d e6 04
eaec: LDA  $04e5,X      ; ad e5 04
eaef: STA  $49          ; 85 49
eaf1: CMP  ($01)        ; d2 01
eaf3: SBC  $04          ; e5 04
eaf5: TXA               ; 8a 
eaf6: STZ  $40          ; 64 40
eaf8: STA  $04e2,X      ; 8d e2 04
eafb: LDA  #$00         ; a9 00
eafd: ADC  $41          ; 65 41
eaff: STA  $04e3,X      ; 8d e3 04
eb02: CMP  ($40)        ; d2 40
eb04: INC  $04,X        ; e6 04
eb06: .byte $c2   ;?
eb07: RTI               ; 40 
eb08: INC  $04,X        ; e6 04
eb0a: LDA  $04e1,X      ; ad e1 04
eb0d: PHA               ; 48 
eb0e: LDA  #$38         ; a9 38
eb10: STA  $04e6,X      ; 8d e6 04
eb13: LDA  $49          ; a5 49
eb15: STA  $04e5,X      ; 8d e5 04
eb18: PLA               ; 68 
eb19: RTS               ; 60 
eb1a: PHX               ; da 
eb1b: JSR  $eae5        ; 20 e5 ea
eb1e: PLX               ; fa 
eb1f: JMP  ($a84a)      ; 4c 4a a8
eb22: LDX  #$00         ; a2 00
eb24: STA  $04e4,X      ; 8d e4 04
eb27: PHA               ; 48 
eb28: PHX               ; da 
eb29: PHP               ; 08 
eb2a: .byte $78   ;?
eb2b: LDA  $04e5,X      ; ad e5 04
eb2e: STA  $49          ; 85 49
eb30: CMP  ($01)        ; d2 01
eb32: SBC  $04          ; e5 04
eb34: .byte $c2   ;?
eb35: BPL  $eb1d        ; 10 e6
eb37: TSB  $d2          ; 04 d2
eb39: BPL  $eb1b        ; 10 e0
eb3b: TSB  $a9          ; 04 a9
eb3d: .byte $02   ;?
eb3e: STA  $4b          ; 85 4b
eb40: LDA  $04e6,X      ; ad e6 04
eb43: AND  #$f8         ; 29 f8
eb45: ORA  $4b          ; 05 4b
eb47: STA  $04e6,X      ; 8d e6 04
eb4a: TXA               ; 8a 
eb4b: STZ  $42          ; 64 42
eb4d: STA  $04e2,X      ; 8d e2 04
eb50: LDA  #$00         ; a9 00
eb52: ADC  $43          ; 65 43
eb54: STA  $04e3,X      ; 8d e3 04
eb57: .byte $c2   ;?
eb58: JSR  $04e6        ; 20 e6 04
eb5b: LDX  #$01         ; a2 01
eb5d: JSR  $a51f        ; 20 1f a5
eb60: CMP  ($20)        ; d2 20
eb62: INC  $04,X        ; e6 04
eb64: LDX  #$3a         ; a2 3a
eb66: DEX               ; ca 
eb67: BNE  $eb66        ; d0 fd
eb69: CMP  ($10)        ; d2 10
eb6b: INC  $04,X        ; e6 04
eb6d: LDA  #$00         ; a9 00
eb6f: STA  $4b          ; 85 4b
eb71: LDA  $04e6,X      ; ad e6 04
eb74: AND  #$f8         ; 29 f8
eb76: ORA  $4b          ; 05 4b
eb78: STA  $04e6,X      ; 8d e6 04
eb7b: LDX  #$4b         ; a2 4b
eb7d: DEX               ; ca 
eb7e: BNE  $eb7d        ; d0 fd
eb80: LDA  $49          ; a5 49
eb82: STA  $04e5,X      ; 8d e5 04
eb85: .byte $c2   ;?
eb86: BPL  $eb68        ; 10 e0
eb88: TSB  $28          ; 04 28
eb8a: PLX               ; fa 
eb8b: PLA               ; 68 
eb8c: RTS               ; 60 
eb8d: PHX               ; da 
eb8e: JSR  $eb22        ; 20 22 eb
eb91: PLX               ; fa 
eb92: JMP  ($a851)      ; 4c 51 a8
eb95: .byte $ff   ;?
eb96: TYA               ; 98 
eb97: .byte $03   ;?
eb98: JMP  ($aea7)      ; 4c a7 ae
eb9b: JMP  ($eae5)      ; 4c e5 ea
eb9e: .byte $ff   ;?
eb9f: TYA               ; 98 
eba0: .byte $03   ;?
eba1: JMP  ($ac1d)      ; 4c 1d ac
eba4: JMP  ($eae7)      ; 4c e7 ea
eba7: .byte $ff   ;?
eba8: TYA               ; 98 
eba9: .byte $03   ;?
ebaa: JMP  ($b106)      ; 4c 06 b1
ebad: JMP  ($eb1a)      ; 4c 1a eb
ebb0: BRA  $eb6d        ; 80 bb
ebb2: CPY  #$5d         ; c0 5d
ebb4: CPX  #$2e         ; e0 2e
ebb6: BVS  $ebcf        ; 70 17
ebb8: CLV               ; b8 
ebb9: .byte $0b   ;?
ebba: TRB  $3b          ; 14 3b
ebbc: PLP               ; 28 
ebbd: ROR  $28,X        ; 76 28
ebbf: ROR  $9a,X        ; 76 9a
ebc1: .byte $99   ;?
ebc2: TXS               ; 9a 
ebc3: .byte $99   ;?
ebc4: BRK               ; 00 
ebc5: .byte $ff   ;?
ebc6: SED               ; f8 
ebc7: ORA  ($79,X)      ; 01 79
ebc9: JSR  $4000        ; 20 00 40
ebcc: BRK               ; 00 
ebcd: BRA  $ebd3        ; 80 04
ebcf: BRK               ; 00 
ebd0: ORA  ($04,X)      ; 01 04
ebd2: BRK               ; 00 
ebd3: PHP               ; 08 
ebd4: BRK               ; 00 
ebd5: PLP               ; 28 
ebd6: .byte $43   ;?
ebd7: AND  #$20         ; 29 20
ebd9: .byte $43   ;?
ebda: .byte $4f   ;?
ebdb: LSR  $5845        ; 4e 45 58
ebde: EOR  ($4e,X)      ; 41 4e
ebe0: TRB  $00          ; 54 00
ebe2: .byte $07   ;?
ebe3: .byte $0f   ;?
ebe4: .byte $0f   ;?
ebe5: .byte $0f   ;?
ebe6: .byte $1f   ;?
ebe7: .byte $1f   ;?
ebe8: .byte $3f   ;?
ebe9: .byte $3f   ;?
ebea: .byte $7f   ;?
ebeb: TSB  $6f          ; 44 6f
ebed: .byte $77   ;?
ebee: ROR  $6f6c        ; 6e 6c 6f
ebf1: ADC  ($64,X)      ; 61 64
ebf3: JSR  $6e69        ; 20 69 6e
ebf6: ADC  #$74         ; 69 74
ebf8: ADC  #$61         ; 69 61
ebfa: STZ  $65          ; 74 65
ebfc: STZ  $20          ; 64 20
ebfe: ROL  $002e        ; 2e 2e 00
ec01: ORA  $2a21,X      ; 0d 21 2a
ec04: LSR  $4449        ; 4e 49 44
ec07: BRK               ; 00 
ec08: EOR  ($b7),Y      ; 51 b7
ec0a: TSX               ; ba 
ec0b: LDY  $41,X        ; b4 41
ec0d: .byte $af   ;?
ec0e: .byte $53   ;?
ec0f: .byte $b7   ;?
ec10: CPY  $b6          ; d4 b6
ec12: .byte $73   ;?
ec13: .byte $b7   ;?
ec14: ORA  $4d4e,X      ; 0d 4e 4d
ec17: .byte $3f   ;?
ec18: AND  $4f45,X      ; 3d 45 4f
ec1b: BRK               ; 00 
ec1c: .byte $e3   ;?
ec1d: LDY  $fc,X        ; b4 fc
ec1f: LDY  $25,X        ; b4 25
ec21: LDA  $6b          ; b5 6b
ec23: LDA  $78          ; b5 78
ec25: LDA  $5d          ; b5 5d
ec27: LDA  $61          ; b5 61
ec29: LDA  $f7          ; b5 f7
ec2b: LDY  $4f,X        ; b4 4f
ec2d: .byte $4b   ;?
ec2e: BRK               ; 00 
ec2f: EOR  $52          ; 45 52
ec31: EOR  ($4f)        ; 52 4f
ec33: EOR  ($00)        ; 52 00
ec35: EOR  $70          ; 55 70
ec37: JMP  ($616f)      ; 6c 6f 61
ec3a: STZ  $20          ; 64 20
ec3c: .byte $63   ;?
ec3d: .byte $6f   ;?
ec3e: ADC  $6c70,X      ; 6d 70 6c
ec41: ADC  $74          ; 65 74
ec43: ADC  $64          ; 65 64
ec45: JSR  $202d        ; 20 2d 20
ec48: BVS  $ecb9        ; 70 6f
ec4a: .byte $77   ;?
ec4b: ADC  $72          ; 65 72
ec4d: JSR  $7963        ; 20 63 79
ec50: .byte $63   ;?
ec51: JMP  ($2065)      ; 6c 65 20
ec54: STZ  $65          ; 64 65
ec56: ROR  $69,X        ; 76 69
ec58: .byte $63   ;?
ec59: ADC  $00          ; 65 00
ec5b: EOR  $70          ; 55 70
ec5d: JMP  ($616f)      ; 6c 6f 61
ec60: STZ  $20          ; 64 20
ec62: ADC  $72          ; 65 72
ec64: ADC  ($6f)        ; 72 6f
ec66: ADC  ($20)        ; 72 20
ec68: AND  $5420,X      ; 2d 20 54
ec6b: ADC  ($79)        ; 72 79
ec6d: JSR  $6761        ; 20 61 67
ec70: ADC  ($69,X)      ; 61 69
ec72: ROR  $4600        ; 6e 00 46
ec75: EOR  ($45)        ; 52 45
ec77: EOR  $4d          ; 45 4d
ec79: EOR  ($4e,X)      ; 41 4e
ec7b: .byte $5f   ;?
ec7c: LSR  $30,X        ; 56 30
ec7e: .byte $33   ;?
ec7f: ROL  $3130        ; 2e 30 31
ec82: ROL  $3030        ; 2e 30 30
ec85: ROL  $3030        ; 2e 30 30
ec88: BRK               ; 00 
ec89: .byte $27   ;?
ec8a: DEX               ; ca 
ec8b: .byte $f3   ;?
ec8c: DEC  $d561        ; ce 61 d5
ec8f: ADC  ($d5,X)      ; 61 d5
ec91: .byte $47   ;?
ec92: DEX               ; ca 
ec93: CPX  ($80ca)      ; fc ca 80
ec96: DEC  $cb7a        ; ce 7a cb
ec99: BRA  $ec69        ; 80 ce
ec9b: AND  ($cc)        ; 32 cc
ec9d: .byte $73   ;?
ec9e: CPY  ($cda1)      ; cc a1 cd
eca1: LDX  #$cd         ; a2 cd
eca3: .byte $83   ;?
eca4: DEC  $cdc7        ; ce c7 cd
eca7: BIT  ($7dce)      ; 2c ce 7d
ecaa: DEC  $ca9a        ; ce 9a ca
ecad: LDY  ($aeca)      ; ac ca ae
ecb0: DEX               ; ca 
ecb1: LDY  $cc          ; a4 cc
ecb3: .byte $d9   ;?
ecb4: CPY  ($cd1b)      ; cc 1b cd
ecb7: STA  $9dcd,X      ; 9d cd 9d
ecba: CMP  $cd9e,X      ; cd 9e cd
ecbd: STZ  $9dcd,X      ; 9e cd 9d
ecc0: CMP  $0112,X      ; cd 12 01
ecc3: BRK               ; 00 
ecc4: .byte $02   ;?
ecc5: BRK               ; 00 
ecc6: BRK               ; 00 
ecc7: BRK               ; 00 
ecc8: RTI               ; 40 
ecc9: ADC  ($05)        ; 72 05
eccb: BRK               ; 00 
eccc: .byte $1b   ;?
eccd: BRK               ; 00 
ecce: .byte $02   ;?
eccf: ORA  ($02,X)      ; 01 02
ecd1: .byte $03   ;?
ecd2: ORA  ($09,X)      ; 01 09
ecd4: .byte $02   ;?
ecd5: .byte $c2   ;?
ecd6: ORA  ($04,X)      ; 01 04
ecd8: ORA  ($00,X)      ; 01 00
ecda: CPY  #$00         ; c0 00
ecdc: ORA  #$04         ; 09 04
ecde: BRK               ; 00 
ecdf: BRK               ; 00 
ece0: BRK               ; 00 
ece1: ORA  ($01,X)      ; 01 01
ece3: BRK               ; 00 
ece4: BRK               ; 00 
ece5: ASL               ; 0a 
ece6: BIT  $01          ; 24 01
ece8: BRK               ; 00 
ece9: ORA  ($a0,X)      ; 01 a0
eceb: BRK               ; 00 
ecec: .byte $02   ;?
eced: ORA  ($02,X)      ; 01 02
ecef: TSB  ($0224)      ; 0c 24 02
ecf2: ORA  ($01,X)      ; 01 01
ecf4: .byte $02   ;?
ecf5: BRK               ; 00 
ecf6: .byte $02   ;?
ecf7: .byte $03   ;?
ecf8: BRK               ; 00 
ecf9: BRK               ; 00 
ecfa: BRK               ; 00 
ecfb: ASL               ; 0a 
ecfc: BIT  $06          ; 24 06
ecfe: .byte $02   ;?
ecff: ORA  ($01,X)      ; 01 01
ed01: EOR  ($02,X)      ; 41 02
ed03: .byte $02   ;?
ed04: BRK               ; 00 
ed05: ORA  #$24         ; 09 24
ed07: .byte $03   ;?
ed08: .byte $03   ;?
ed09: ORA  ($01,X)      ; 01 01
ed0b: BRK               ; 00 
ed0c: BPL  $ed0e        ; 10 00
ed0e: PHP               ; 08 
ed0f: BIT  $05          ; 24 05
ed11: BPL  $ed15        ; 10 02
ed13: .byte $02   ;?
ed14: ORA  ($00),Y      ; 11 00
ed16: ASL               ; 0a 
ed17: BIT  $06          ; 24 06
ed19: ORA  ($12),Y      ; 11 12
ed1b: ORA  ($01,X)      ; 01 01
ed1d: .byte $02   ;?
ed1e: .byte $02   ;?
ed1f: BRK               ; 00 
ed20: TSB  ($0224)      ; 0c 24 02
ed23: ORA  ($03)        ; 12 03
ed25: ASL  $00,X        ; 06 00
ed27: .byte $02   ;?
ed28: .byte $03   ;?
ed29: BRK               ; 00 
ed2a: BRK               ; 00 
ed2b: BRK               ; 00 
ed2c: TSB  ($0224)      ; 0c 24 02
ed2f: TSB  $01          ; 04 01
ed31: ORA  ($00,X)      ; 01 00
ed33: .byte $02   ;?
ed34: .byte $03   ;?
ed35: BRK               ; 00 
ed36: BRK               ; 00 
ed37: BRK               ; 00 
ed38: ASL               ; 0a 
ed39: BIT  $06          ; 24 06
ed3b: ORA  $0c          ; 05 0c
ed3d: ORA  ($01,X)      ; 01 01
ed3f: .byte $02   ;?
ed40: .byte $02   ;?
ed41: BRK               ; 00 
ed42: ORA  #$24         ; 09 24
ed44: .byte $03   ;?
ed45: ASL  $01,X        ; 06 01
ed47: .byte $03   ;?
ed48: BRK               ; 00 
ed49: ORA  $00          ; 05 00
ed4b: TSB  ($0224)      ; 0c 24 02
ed4e: ASL               ; 0a 
ed4f: ORA  ($02,X)      ; 01 02
ed51: BRK               ; 00 
ed52: .byte $02   ;?
ed53: .byte $03   ;?
ed54: BRK               ; 00 
ed55: BRK               ; 00 
ed56: ASL  $0a,X        ; 06 0a
ed58: BIT  $06          ; 24 06
ed5a: .byte $0b   ;?
ed5b: ASL               ; 0a 
ed5c: ORA  ($03,X)      ; 01 03
ed5e: BRK               ; 00 
ed5f: BRK               ; 00 
ed60: BRK               ; 00 
ed61: TSB  ($0224)      ; 0c 24 02
ed64: ORA  $0603,X      ; 0d 03 06
ed67: BRK               ; 00 
ed68: .byte $02   ;?
ed69: .byte $03   ;?
ed6a: BRK               ; 00 
ed6b: BRK               ; 00 
ed6c: BRK               ; 00 
ed6d: ASL               ; 0a 
ed6e: BIT  $06          ; 24 06
ed70: ASL  $010d        ; 0e 0d 01
ed73: ORA  ($02,X)      ; 01 02
ed75: .byte $02   ;?
ed76: BRK               ; 00 
ed77: ASL  $0424        ; 0e 24 04
ed7a: TSB  ($0403)      ; 0c 03 04
ed7d: .byte $0b   ;?
ed7e: ASL  $0302        ; 0e 02 03
ed81: BRK               ; 00 
ed82: BRK               ; 00 
ed83: BRK               ; 00 
ed84: BRK               ; 00 
ed85: ORA  #$04         ; 09 04
ed87: ORA  ($00,X)      ; 01 00
ed89: BRK               ; 00 
ed8a: ORA  ($02,X)      ; 01 02
ed8c: BRK               ; 00 
ed8d: BRK               ; 00 
ed8e: ORA  #$04         ; 09 04
ed90: ORA  ($01,X)      ; 01 01
ed92: ORA  ($01,X)      ; 01 01
ed94: .byte $02   ;?
ed95: BRK               ; 00 
ed96: BRK               ; 00 
ed97: .byte $07   ;?
ed98: BIT  $01          ; 24 01
ed9a: .byte $03   ;?
ed9b: ORA  ($01,X)      ; 01 01
ed9d: BRK               ; 00 
ed9e: .byte $17   ;?
ed9f: BIT  $02          ; 24 02
eda1: ORA  ($02,X)      ; 01 02
eda3: .byte $02   ;?
eda4: BPL  $edab        ; 10 05
eda6: RTI               ; 40 
eda7: .byte $1f   ;?
eda8: BRK               ; 00 
eda9: BRA  $ede9        ; 80 3e
edab: BRK               ; 00 
edac: BRK               ; 00 
edad: ADC  $4400,X      ; 7d 00 44
edb0: LDY  ($8000)      ; ac 00 80
edb3: .byte $bb   ;?
edb4: BRK               ; 00 
edb5: ORA  #$05         ; 09 05
edb7: STA  ($05,X)      ; 81 05
edb9: CPY  #$00         ; c0 00
edbb: ORA  ($00,X)      ; 01 00
edbd: BRK               ; 00 
edbe: .byte $07   ;?
edbf: AND  $01          ; 25 01
edc1: ORA  ($00,X)      ; 01 00
edc3: BRK               ; 00 
edc4: BRK               ; 00 
edc5: ORA  #$04         ; 09 04
edc7: ORA  ($02,X)      ; 01 02
edc9: ORA  ($01,X)      ; 01 01
edcb: .byte $02   ;?
edcc: BRK               ; 00 
edcd: BRK               ; 00 
edce: .byte $07   ;?
edcf: BIT  $01          ; 24 01
edd1: .byte $03   ;?
edd2: ORA  ($01,X)      ; 01 01
edd4: BRK               ; 00 
edd5: .byte $17   ;?
edd6: BIT  $02          ; 24 02
edd8: ORA  ($02,X)      ; 01 02
edda: .byte $03   ;?
eddb: CLC               ; 18 
eddc: ORA  $40          ; 05 40
edde: .byte $1f   ;?
eddf: BRK               ; 00 
ede0: BRA  $ee20        ; 80 3e
ede2: BRK               ; 00 
ede3: BRK               ; 00 
ede4: ADC  $4400,X      ; 7d 00 44
ede7: LDY  ($8000)      ; ac 00 80
edea: .byte $bb   ;?
edeb: BRK               ; 00 
edec: ORA  #$05         ; 09 05
edee: STA  ($05,X)      ; 81 05
edf0: JSR  $0101        ; 20 01 01
edf3: BRK               ; 00 
edf4: BRK               ; 00 
edf5: .byte $07   ;?
edf6: AND  $01          ; 25 01
edf8: ORA  ($00,X)      ; 01 00
edfa: BRK               ; 00 
edfb: BRK               ; 00 
edfc: ORA  #$04         ; 09 04
edfe: .byte $02   ;?
edff: BRK               ; 00 
ee00: BRK               ; 00 
ee01: ORA  ($02,X)      ; 01 02
ee03: BRK               ; 00 
ee04: BRK               ; 00 
ee05: ORA  #$04         ; 09 04
ee07: .byte $02   ;?
ee08: ORA  ($01,X)      ; 01 01
ee0a: ORA  ($02,X)      ; 01 02
ee0c: BRK               ; 00 
ee0d: BRK               ; 00 
ee0e: .byte $07   ;?
ee0f: BIT  $01          ; 24 01
ee11: TSB  $03          ; 04 03
ee13: ORA  ($00,X)      ; 01 00
ee15: ORA  $0224,X      ; 1d 24 02
ee18: ORA  ($02,X)      ; 01 02
ee1a: .byte $02   ;?
ee1b: BPL  $ee24        ; 10 07
ee1d: RTI               ; 40 
ee1e: .byte $1f   ;?
ee1f: BRK               ; 00 
ee20: BRA  $ee60        ; 80 3e
ee22: BRK               ; 00 
ee23: BRK               ; 00 
ee24: ADC  $4400,X      ; 7d 00 44
ee27: LDY  ($8000)      ; ac 00 80
ee2a: .byte $bb   ;?
ee2b: BRK               ; 00 
ee2c: BRK               ; 00 
ee2d: .byte $77   ;?
ee2e: ORA  ($00,X)      ; 01 00
ee30: INC  $0902        ; ee 02 09
ee33: ORA  $01          ; 05 01
ee35: ORA  #$00         ; 09 00
ee37: .byte $03   ;?
ee38: ORA  ($00,X)      ; 01 00
ee3a: BRK               ; 00 
ee3b: .byte $07   ;?
ee3c: AND  $01          ; 25 01
ee3e: ORA  ($01,X)      ; 01 01
ee40: TSB  $00          ; 04 00
ee42: ORA  #$04         ; 09 04
ee44: .byte $02   ;?
ee45: .byte $02   ;?
ee46: ORA  ($01,X)      ; 01 01
ee48: .byte $02   ;?
ee49: BRK               ; 00 
ee4a: BRK               ; 00 
ee4b: .byte $07   ;?
ee4c: BIT  $01          ; 24 01
ee4e: TSB  $01          ; 04 01
ee50: ORA  ($00,X)      ; 01 00
ee52: INA               ; 1a 
ee53: BIT  $02          ; 24 02
ee55: ORA  ($02,X)      ; 01 02
ee57: .byte $03   ;?
ee58: CLC               ; 18 
ee59: ASL  $40,X        ; 06 40
ee5b: .byte $1f   ;?
ee5c: BRK               ; 00 
ee5d: BRA  $ee9d        ; 80 3e
ee5f: BRK               ; 00 
ee60: BRK               ; 00 
ee61: ADC  $4400,X      ; 7d 00 44
ee64: LDY  ($8000)      ; ac 00 80
ee67: .byte $bb   ;?
ee68: BRK               ; 00 
ee69: BRK               ; 00 
ee6a: .byte $77   ;?
ee6b: ORA  ($09,X)      ; 01 09
ee6d: ORA  $01          ; 05 01
ee6f: ORA  #$40         ; 09 40
ee71: .byte $02   ;?
ee72: ORA  ($00,X)      ; 01 00
ee74: BRK               ; 00 
ee75: .byte $07   ;?
ee76: AND  $01          ; 25 01
ee78: ORA  ($01,X)      ; 01 01
ee7a: TSB  $00          ; 04 00
ee7c: ORA  #$04         ; 09 04
ee7e: .byte $03   ;?
ee7f: BRK               ; 00 
ee80: ORA  ($03,X)      ; 01 03
ee82: BRK               ; 00 
ee83: BRK               ; 00 
ee84: BRK               ; 00 
ee85: ORA  #$21         ; 09 21
ee87: ORA  ($01),Y      ; 11 01
ee89: BRK               ; 00 
ee8a: ORA  ($22,X)      ; 01 22
ee8c: .byte $78   ;?
ee8d: BRK               ; 00 
ee8e: .byte $07   ;?
ee8f: ORA  $84          ; 05 84
ee91: .byte $03   ;?
ee92: .byte $23   ;?
ee93: BRK               ; 00 
ee94: ORA  ($07,X)      ; 01 07
ee96: INC  $c186        ; fe 86 c1
ee99: BIT  #$48         ; 89 48
ee9b: EOR  $b1b5,X      ; 4d b5 b1
ee9e: STY  $c5          ; 84 c5
eea0: ASL  $2d,X        ; 16 2d
eea2: LSR               ; 4a 
eea3: .byte $d3   ;?
eea4: TRB  $3c          ; 14 3c
eea6: BRK               ; 00 
eea7: BRK               ; 00 
eea8: ORA  ($00,X)      ; 01 00
eeaa: BRK               ; 00 
eeab: BRK               ; 00 
eeac: BRK               ; 00 
eead: BRK               ; 00 
eeae: BRK               ; 00 
eeaf: SBC  #$dd         ; e9 dd
eeb1: .byte $17   ;?
eeb2: .byte $22   ;?
eeb3: BVC  $eeb5        ; 50 00
eeb5: JMP  ($021d)      ; 4c 1d 02
eeb8: BRK               ; 00 
eeb9: BRK               ; 00 
eeba: BRK               ; 00 
eebb: BRK               ; 00 
eebc: BRK               ; 00 
eebd: DEC  $00ff        ; ce ff 00
eec0: BRK               ; 00 
eec1: BRK               ; 00 
eec2: BRK               ; 00 
eec3: BRK               ; 00 
eec4: BRK               ; 00 
eec5: BRK               ; 00 
eec6: BRK               ; 00 
eec7: BRK               ; 00 
eec8: BRK               ; 00 
eec9: AND  ($00)        ; 32 00
eecb: BRK               ; 00 
eecc: BRK               ; 00 
eecd: BRK               ; 00 
eece: BRK               ; 00 
eecf: BRK               ; 00 
eed0: BRK               ; 00 
eed1: ORA  $0c          ; 05 0c
eed3: ORA  #$01         ; 09 01
eed5: LDA  ($01,X)      ; a1 01
eed7: STA  $01          ; 85 01
eed9: ORA  $00          ; 15 00
eedb: AND  $01          ; 25 01
eedd: ORA  #$e9         ; 09 e9
eedf: ORA  #$ea         ; 09 ea
eee1: ADC  $01          ; 75 01
eee3: STA  $02          ; 95 02
eee5: STA  ($02,X)      ; 81 02
eee7: ORA  #$e2         ; 09 e2
eee9: STA  $01          ; 95 01
eeeb: STA  ($06,X)      ; 81 06
eeed: ORA  #$00         ; 09 00
eeef: STA  $02          ; 95 02
eef1: STA  ($03,X)      ; 81 03
eef3: ORA  #$36         ; 09 36
eef5: LDA  ($02,X)      ; a1 02
eef7: ORA  $09          ; 05 09
eef9: .byte $19   ;?
eefa: ORA  ($29,X)      ; 01 29
eefc: .byte $02   ;?
eefd: ADC  $02          ; 75 02
eeff: STA  $01          ; 95 01
ef01: ORA  $01          ; 15 01
ef03: AND  $02          ; 25 02
ef05: STA  ($40,X)      ; 81 40
ef07: CPY  #$05         ; c0 05
ef09: TSB  ($0009)      ; 0c 09 00
ef0c: ORA  $00          ; 15 00
ef0e: AND  $01          ; 25 01
ef10: ADC  $01          ; 75 01
ef12: STA  $01          ; 95 01
ef14: STA  ($03,X)      ; 81 03
ef16: STA  $02          ; 85 02
ef18: ORA  $0c          ; 05 0c
ef1a: ORA  #$00         ; 09 00
ef1c: STA  $10          ; 95 10
ef1e: STA  ($02,X)      ; 81 02
ef20: STA  $03          ; 85 03
ef22: ORA  #$00         ; 09 00
ef24: STA  ($02),Y      ; 91 02
ef26: STA  $04          ; 85 04
ef28: ORA  #$00         ; 09 00
ef2a: ADC  $08          ; 75 08
ef2c: STA  $26          ; 95 26
ef2e: STA  ($02),Y      ; 91 02
ef30: STA  $05          ; 85 05
ef32: ORA  #$00         ; 09 00
ef34: STA  $22          ; 95 22
ef36: STA  ($02,X)      ; 81 02
ef38: STA  $06          ; 85 06
ef3a: ORA  #$00         ; 09 00
ef3c: STA  $24          ; 95 24
ef3e: STA  ($02),Y      ; 91 02
ef40: STA  $07          ; 85 07
ef42: ORA  #$00         ; 09 00
ef44: STA  $20          ; 95 20
ef46: STA  ($02,X)      ; 81 02
ef48: CPY  #$05         ; c0 05
ef4a: .byte $0b   ;?
ef4b: ORA  #$01         ; 09 01
ef4d: LDA  ($01,X)      ; a1 01
ef4f: LDA  ($02,X)      ; a1 02
ef51: STA  $08          ; 85 08
ef53: ORA  $08          ; 05 08
ef55: ORA  $00          ; 15 00
ef57: AND  $01          ; 25 01
ef59: ADC  $01          ; 75 01
ef5b: STA  $04          ; 95 04
ef5d: ORA  #$00         ; 09 00
ef5f: ORA  #$17         ; 09 17
ef61: ORA  #$00         ; 09 00
ef63: ORA  #$00         ; 09 00
ef65: STA  ($02),Y      ; 91 02
ef67: STA  $04          ; 95 04
ef69: STA  ($03),Y      ; 91 03
ef6b: CPY  #$a1         ; c0 a1
ef6d: ORA  ($85,X)      ; 01 85
ef6f: PHP               ; 08 
ef70: ORA  $0b          ; 05 0b
ef72: AND  $01          ; 25 01
ef74: ADC  $01          ; 75 01
ef76: STA  $04          ; 95 04
ef78: ORA  #$2f         ; 09 2f
ef7a: ORA  #$20         ; 09 20
ef7c: ORA  #$70         ; 09 70
ef7e: ORA  #$00         ; 09 00
ef80: STA  ($02,X)      ; 81 02
ef82: ASL  $99,X        ; 06 99
ef84: .byte $ff   ;?
ef85: STA  $01          ; 95 01
ef87: ASL               ; 0a 
ef88: ASL  $81ff        ; 1e ff 81
ef8b: .byte $02   ;?
ef8c: STA  $02          ; 95 02
ef8e: STA  ($03,X)      ; 81 03
ef90: ORA  #$07         ; 09 07
ef92: ORA  $09          ; 05 09
ef94: ORA  #$01         ; 09 01
ef96: STA  $01          ; 95 01
ef98: STA  ($02,X)      ; 81 02
ef9a: ORA  $0b          ; 05 0b
ef9c: ORA  #$06         ; 09 06
ef9e: LDA  ($02,X)      ; a1 02
efa0: .byte $19   ;?
efa1: BCS  $efcc        ; b0 29
efa3: LDY  ($0115)      ; bc 15 01
efa6: AND  $0d          ; 25 0d
efa8: ADC  $04          ; 75 04
efaa: STA  $01          ; 95 01
efac: STA  ($00,X)      ; 81 00
efae: CPY  #$75         ; c0 75
efb0: TSB  $81          ; 04 81
efb2: .byte $03   ;?
efb3: ASL  $99,X        ; 06 99
efb5: .byte $ff   ;?
efb6: ORA  #$00         ; 09 00
efb8: ORA  #$00         ; 09 00
efba: ORA  $00          ; 15 00
efbc: AND  $01          ; 25 01
efbe: ADC  $01          ; 75 01
efc0: STA  $02          ; 95 02
efc2: STA  ($02,X)      ; 81 02
efc4: STA  $06          ; 95 06
efc6: STA  ($03,X)      ; 81 03
efc8: ORA  #$62         ; 09 62
efca: ADC  $08          ; 75 08
efcc: STA  $04          ; 95 04
efce: STA  ($02,X)      ; 81 02
efd0: CPY  #$c0         ; c0 c0
efd2: ASL  $99,X        ; 06 99
efd4: .byte $ff   ;?
efd5: ORA  #$01         ; 09 01
efd7: LDA  ($01,X)      ; a1 01
efd9: ASL               ; 0a 
efda: BRK               ; 00 
efdb: .byte $ff   ;?
efdc: LDA  ($02,X)      ; a1 02
efde: STA  $11          ; 85 11
efe0: ORA  $00          ; 15 00
efe2: .byte $27   ;?
efe3: .byte $ff   ;?
efe4: .byte $ff   ;?
efe5: BRK               ; 00 
efe6: BRK               ; 00 
efe7: ADC  $10          ; 75 10
efe9: STA  $01          ; 95 01
efeb: ASL               ; 0a 
efec: ORA  ($ff,X)      ; 01 ff
efee: LDA  ($03),Y      ; b1 03
eff0: ROL  $ff,X        ; 26 ff
eff2: BRK               ; 00 
eff3: STA  $02          ; 95 02
eff5: ADC  $08          ; 75 08
eff7: ASL               ; 0a 
eff8: .byte $02   ;?
eff9: .byte $ff   ;?
effa: LDA  ($03),Y      ; b1 03
effc: CPY  #$09         ; c0 09
effe: JSR  $02a1        ; 20 a1 02
f001: STA  $12          ; 85 12
f003: ORA  #$35         ; 09 35
f005: ORA  #$36         ; 09 36
f007: ORA  $00          ; 15 00
f009: ROL  $ff,X        ; 26 ff
f00b: BRK               ; 00 
f00c: ADC  $08          ; 75 08
f00e: STA  $02          ; 95 02
f010: LDA  ($03),Y      ; b1 03
f012: .byte $19   ;?
f013: STA  ($29,X)      ; 81 29
f015: TXA               ; 8a 
f016: AND  $01          ; 25 01
f018: ADC  $01          ; 75 01
f01a: STA  $0a          ; 95 0a
f01c: LDA  ($03),Y      ; b1 03
f01e: ADC  $01          ; 75 01
f020: STA  $06          ; 95 06
f022: LDA  ($03),Y      ; b1 03
f024: CPY  #$09         ; c0 09
f026: BIT  $a1          ; 24 a1
f028: .byte $02   ;?
f029: STA  $13          ; 85 13
f02b: ORA  #$26         ; 09 26
f02d: ORA  #$25         ; 09 25
f02f: ASL               ; 0a 
f030: BPL  $f031        ; 10 ff
f032: ORA  $00          ; 15 00
f034: AND  $01          ; 25 01
f036: ADC  $01          ; 75 01
f038: STA  $03          ; 95 03
f03a: STA  ($02),Y      ; 91 02
f03c: ASL               ; 0a 
f03d: ORA  ($ff),Y      ; 11 ff
f03f: AND  $0f          ; 25 0f
f041: ADC  $04          ; 75 04
f043: STA  $01          ; 95 01
f045: STA  ($02),Y      ; 91 02
f047: ADC  $01          ; 75 01
f049: STA  ($03),Y      ; 91 03
f04b: CPY  #$09         ; c0 09
f04d: PHA               ; 48 
f04e: LDA  ($02,X)      ; a1 02
f050: STA  $14          ; 85 14
f052: .byte $19   ;?
f053: STA  ($29,X)      ; 81 29
f055: TXA               ; 8a 
f056: ORA  $01          ; 15 01
f058: AND  $0a          ; 25 0a
f05a: ADC  $04          ; 75 04
f05c: STA  $01          ; 95 01
f05e: STA  ($00),Y      ; 91 00
f060: ADC  $04          ; 75 04
f062: STA  $01          ; 95 01
f064: STA  ($03),Y      ; 91 03
f066: ASL               ; 0a 
f067: .byte $23   ;?
f068: .byte $ff   ;?
f069: ORA  $00          ; 15 00
f06b: AND  $03          ; 25 03
f06d: ADC  $02          ; 75 02
f06f: STA  $01          ; 95 01
f071: STA  ($02),Y      ; 91 02
f073: ADC  $05          ; 75 05
f075: STA  $01          ; 95 01
f077: STA  ($03),Y      ; 91 03
f079: ASL               ; 0a 
f07a: .byte $22   ;?
f07b: .byte $ff   ;?
f07c: AND  $01          ; 25 01
f07e: ADC  $01          ; 75 01
f080: STA  $01          ; 95 01
f082: STA  ($02),Y      ; 91 02
f084: CPY  #$09         ; c0 09
f086: .byte $2b   ;?
f087: LDA  ($02,X)      ; a1 02
f089: STA  $15          ; 85 15
f08b: ADC  $07          ; 75 07
f08d: STA  $01          ; 95 01
f08f: STA  ($03),Y      ; 91 03
f091: ASL               ; 0a 
f092: BIT  $ff          ; 24 ff
f094: ORA  $00          ; 15 00
f096: AND  $01          ; 25 01
f098: ADC  $01          ; 75 01
f09a: STA  $01          ; 95 01
f09c: STA  ($02),Y      ; 91 02
f09e: ASL               ; 0a 
f09f: BIT  ($27ff)      ; 2c ff 27
f0a2: .byte $ff   ;?
f0a3: .byte $ff   ;?
f0a4: BRK               ; 00 
f0a5: BRK               ; 00 
f0a6: ADC  $10          ; 75 10
f0a8: STA  $08          ; 95 08
f0aa: STA  ($02),Y      ; 91 02
f0ac: CPY  #$0a         ; c0 0a
f0ae: .byte $17   ;?
f0af: .byte $ff   ;?
f0b0: LDA  ($02,X)      ; a1 02
f0b2: STA  $16          ; 85 16
f0b4: ASL               ; 0a 
f0b5: CLC               ; 18 
f0b6: .byte $ff   ;?
f0b7: ORA  $00          ; 15 00
f0b9: AND  $0f          ; 25 0f
f0bb: ADC  $04          ; 75 04
f0bd: STA  $01          ; 95 01
f0bf: STA  ($02),Y      ; 91 02
f0c1: ADC  $04          ; 75 04
f0c3: STA  ($03),Y      ; 91 03
f0c5: ASL               ; 0a 
f0c6: INA               ; 1a 
f0c7: .byte $ff   ;?
f0c8: ASL               ; 0a 
f0c9: .byte $1b   ;?
f0ca: .byte $ff   ;?
f0cb: ASL               ; 0a 
f0cc: .byte $1f   ;?
f0cd: .byte $ff   ;?
f0ce: ASL               ; 0a 
f0cf: JSR  $25ff        ; 20 ff 25
f0d2: ORA  ($75,X)      ; 01 75
f0d4: ORA  ($95,X)      ; 01 95
f0d6: TSB  $91          ; 04 91
f0d8: .byte $02   ;?
f0d9: ADC  $02          ; 75 02
f0db: STA  $01          ; 95 01
f0dd: ORA  $ff          ; 15 ff
f0df: AND  $01          ; 25 01
f0e1: ASL               ; 0a 
f0e2: TRB  ($91ff)      ; 1c ff 91
f0e5: ROL  $75,X        ; 26 75
f0e7: .byte $02   ;?
f0e8: STA  ($03),Y      ; 91 03
f0ea: CPY  #$a1         ; c0 a1
f0ec: .byte $02   ;?
f0ed: STA  $17          ; 85 17
f0ef: ORA  $00          ; 15 00
f0f1: .byte $27   ;?
f0f2: .byte $ff   ;?
f0f3: .byte $ff   ;?
f0f4: BRK               ; 00 
f0f5: BRK               ; 00 
f0f6: ADC  $10          ; 75 10
f0f8: STA  $01          ; 95 01
f0fa: ORA  #$64         ; 09 64
f0fc: LDA  ($02),Y      ; b1 02
f0fe: ROL  $ff,X        ; 26 ff
f100: BRK               ; 00 
f101: ADC  $08          ; 75 08
f103: STA  $02          ; 95 02
f105: ORA  #$65         ; 09 65
f107: LDA  ($02),Y      ; b1 02
f109: CPY  #$c0         ; c0 c0
f10b: ORA  $21f1,X      ; 1d f1 21
f10e: SBC  ($33),Y      ; f1 33
f110: SBC  ($59),Y      ; f1 59
f112: SBC  ($91),Y      ; f1 91
f114: SBC  ($a3),Y      ; f1 a3
f116: SBC  ($cb),Y      ; f1 cb
f118: SBC  ($dd),Y      ; f1 dd
f11a: SBC  ($f3),Y      ; f1 f3
f11c: SBC  ($04),Y      ; f1 04
f11e: .byte $03   ;?
f11f: ORA  #$04         ; 09 04
f121: ORA  ($03)        ; 12 03
f123: .byte $43   ;?
f124: BRK               ; 00 
f125: .byte $6f   ;?
f126: BRK               ; 00 
f127: ROR  $6500        ; 6e 00 65
f12a: BRK               ; 00 
f12b: .byte $78   ;?
f12c: BRK               ; 00 
f12d: ADC  ($00,X)      ; 61 00
f12f: ROR  $7400        ; 6e 00 74
f132: BRK               ; 00 
f133: ROL  $03,X        ; 26 03
f135: PHA               ; 48 
f136: BRK               ; 00 
f137: ADC  #$00         ; 69 00
f139: JSR  $5200        ; 20 00 52
f13c: BRK               ; 00 
f13d: ADC  $00          ; 65 00
f13f: .byte $73   ;?
f140: BRK               ; 00 
f141: JSR  $5500        ; 20 00 55
f144: BRK               ; 00 
f145: .byte $53   ;?
f146: BRK               ; 00 
f147: .byte $42   ;?
f148: BRK               ; 00 
f149: AND  $4300,X      ; 2d 00 43
f14c: BRK               ; 00 
f14d: JSR  $4100        ; 20 00 41
f150: BRK               ; 00 
f151: EOR  $00          ; 55 00
f153: TSB  $00          ; 44 00
f155: EOR  #$00         ; 49 00
f157: .byte $4f   ;?
f158: BRK               ; 00 
f159: INA               ; 1a 
f15a: .byte $03   ;?
f15b: BMI  $f15d        ; 30 00
f15d: BMI  $f15f        ; 30 00
f15f: BMI  $f161        ; 30 00
f161: BMI  $f163        ; 30 00
f163: BMI  $f165        ; 30 00
f165: BMI  $f167        ; 30 00
f167: BMI  $f169        ; 30 00
f169: BMI  $f16b        ; 30 00
f16b: BMI  $f16d        ; 30 00
f16d: BMI  $f16f        ; 30 00
f16f: BMI  $f171        ; 30 00
f171: BMI  $f173        ; 30 00
f173: ASL  $5503        ; 1e 03 55
f176: BRK               ; 00 
f177: .byte $43   ;?
f178: BRK               ; 00 
f179: EOR  ($00),Y      ; 51 00
f17b: AND  ($00),Y      ; 31 00
f17d: AND  ($00),Y      ; 31 00
f17f: BMI  $f181        ; 30 00
f181: BMI  $f183        ; 30 00
f183: AND  ($00),Y      ; 31 00
f185: BMI  $f187        ; 30 00
f187: BMI  $f189        ; 30 00
f189: BMI  $f18b        ; 30 00
f18b: BMI  $f18d        ; 30 00
f18d: BMI  $f18f        ; 30 00
f18f: AND  ($00),Y      ; 31 00
f191: ORA  ($03)        ; 12 03
f193: AND  ($00),Y      ; 31 00
f195: AND  ($00)        ; 32 00
f197: AND  ($00),Y      ; 31 00
f199: AND  $00          ; 35 00
f19b: AND  ($00)        ; 32 00
f19d: BMI  $f19f        ; 30 00
f19f: AND  ($00),Y      ; 31 00
f1a1: ROL  $00,X        ; 36 00
f1a3: PLP               ; 28 
f1a4: .byte $03   ;?
f1a5: .byte $43   ;?
f1a6: BRK               ; 00 
f1a7: .byte $6f   ;?
f1a8: BRK               ; 00 
f1a9: ADC  $6d00,X      ; 6d 00 6d
f1ac: BRK               ; 00 
f1ad: ADC  $00          ; 75 00
f1af: ROR  $6900        ; 6e 00 69
f1b2: BRK               ; 00 
f1b3: .byte $63   ;?
f1b4: BRK               ; 00 
f1b5: ADC  ($00,X)      ; 61 00
f1b7: STZ  $00          ; 74 00
f1b9: ADC  #$00         ; 69 00
f1bb: .byte $6f   ;?
f1bc: BRK               ; 00 
f1bd: ROR  $2000        ; 6e 00 20
f1c0: BRK               ; 00 
f1c1: EOR  ($00,X)      ; 41 00
f1c3: ADC  $00          ; 75 00
f1c5: STZ  $00          ; 64 00
f1c7: ADC  #$00         ; 69 00
f1c9: .byte $6f   ;?
f1ca: BRK               ; 00 
f1cb: ORA  ($03)        ; 12 03
f1cd: .byte $53   ;?
f1ce: BRK               ; 00 
f1cf: ADC  #$00         ; 69 00
f1d1: STZ  $00          ; 64 00
f1d3: ADC  $00          ; 65 00
f1d5: STZ  $00          ; 74 00
f1d7: .byte $6f   ;?
f1d8: BRK               ; 00 
f1d9: ROR  $6500        ; 6e 00 65
f1dc: BRK               ; 00 
f1dd: ASL  $03,X        ; 16 03
f1df: EOR  $6900,X      ; 4d 00 69
f1e2: BRK               ; 00 
f1e3: .byte $63   ;?
f1e4: BRK               ; 00 
f1e5: ADC  ($00)        ; 72 00
f1e7: .byte $6f   ;?
f1e8: BRK               ; 00 
f1e9: BVS  $f1eb        ; 70 00
f1eb: PLA               ; 68 
f1ec: BRK               ; 00 
f1ed: .byte $6f   ;?
f1ee: BRK               ; 00 
f1ef: ROR  $6500        ; 6e 00 65
f1f2: BRK               ; 00 
f1f3: ORA  ($03)        ; 12 03
f1f5: .byte $53   ;?
f1f6: BRK               ; 00 
f1f7: BVS  $f1f9        ; 70 00
f1f9: ADC  $00          ; 65 00
f1fb: ADC  ($00,X)      ; 61 00
f1fd: .byte $6b   ;?
f1fe: BRK               ; 00 
f1ff: ADC  $00          ; 65 00
f201: ADC  ($00)        ; 72 00
f203: .byte $73   ;?
f204: BRK               ; 00 
f205: ROL               ; 2a 
f206: .byte $03   ;?
f207: LSR  $00,X        ; 46 00
f209: EOR  ($00)        ; 52 00
f20b: EOR  $00          ; 45 00
f20d: EOR  $00          ; 45 00
f20f: EOR  $4100,X      ; 4d 00 41
f212: BRK               ; 00 
f213: LSR  $5f00        ; 4e 00 5f
f216: BRK               ; 00 
f217: LSR  $00,X        ; 56 00
f219: BMI  $f21b        ; 30 00
f21b: .byte $33   ;?
f21c: BRK               ; 00 
f21d: ROL  $3000        ; 2e 00 30
f220: BRK               ; 00 
f221: AND  ($00),Y      ; 31 00
f223: ROL  $3000        ; 2e 00 30
f226: BRK               ; 00 
f227: BMI  $f229        ; 30 00
f229: ROL  $3000        ; 2e 00 30
f22c: BRK               ; 00 
f22d: BMI  $f22f        ; 30 00
f22f: BRK               ; 00 
f230: ORA  ($01,X)      ; 01 01
f232: BRK               ; 00 
f233: ORA  ($00,X)      ; 01 00
f235: ORA  $ff          ; 05 ff
f237: ROR  $81d1        ; 6e d1 81
f23a: BRK               ; 00 
f23b: ORA  ($00,X)      ; 01 00
f23d: ORA  $00          ; 05 00
f23f: BPL  $f213        ; 10 d2
f241: ORA  ($00,X)      ; 01 00
f243: .byte $02   ;?
f244: BRK               ; 00 
f245: ORA  $ff          ; 05 ff
f247: INC  $81cf        ; fe cf 81
f24a: BRK               ; 00 
f24b: .byte $02   ;?
f24c: BRK               ; 00 
f24d: ORA  $02          ; 05 02
f24f: ROR               ; 6a 
f250: ORA  ($82)        ; 12 82
f252: BRK               ; 00 
f253: .byte $02   ;?
f254: BRK               ; 00 
f255: ORA  $02          ; 05 02
f257: RTS               ; 60 
f258: BRK               ; 00 
f259: .byte $83   ;?
f25a: BRK               ; 00 
f25b: .byte $02   ;?
f25c: BRK               ; 00 
f25d: ORA  $02          ; 05 02
f25f: .byte $62   ;?
f260: BRK               ; 00 
f261: STY  $00          ; 84 00
f263: .byte $02   ;?
f264: BRK               ; 00 
f265: ORA  $02          ; 05 02
f267: STZ  $00          ; 64 00
f269: ORA  ($01,X)      ; 01 01
f26b: .byte $02   ;?
f26c: BRK               ; 00 
f26d: ORA  $ff          ; 05 ff
f26f: .byte $a3   ;?
f270: BNE  $f1f3        ; d0 81
f272: ORA  ($02,X)      ; 01 02
f274: BRK               ; 00 
f275: ORA  $02          ; 05 02
f277: ROR               ; 6a 
f278: ORA  ($82)        ; 12 82
f27a: ORA  ($02,X)      ; 01 02
f27c: BRK               ; 00 
f27d: ORA  $02          ; 05 02
f27f: RTS               ; 60 
f280: BRK               ; 00 
f281: .byte $83   ;?
f282: ORA  ($02,X)      ; 01 02
f284: BRK               ; 00 
f285: ORA  $02          ; 05 02
f287: .byte $62   ;?
f288: BRK               ; 00 
f289: STY  $01          ; 84 01
f28b: .byte $02   ;?
f28c: BRK               ; 00 
f28d: ORA  $02          ; 05 02
f28f: STZ  $00          ; 64 00
f291: ORA  ($02,X)      ; 01 02
f293: .byte $02   ;?
f294: BRK               ; 00 
f295: ORA  $ff          ; 05 ff
f297: LDA  ($d0),Y      ; b1 d0
f299: STA  ($02,X)      ; 81 02
f29b: .byte $02   ;?
f29c: BRK               ; 00 
f29d: ORA  $02          ; 05 02
f29f: JMP  ($8212)      ; 6c 12 82
f2a2: .byte $02   ;?
f2a3: .byte $02   ;?
f2a4: BRK               ; 00 
f2a5: ORA  $02          ; 05 02
f2a7: RTS               ; 60 
f2a8: BRK               ; 00 
f2a9: .byte $83   ;?
f2aa: .byte $02   ;?
f2ab: .byte $02   ;?
f2ac: BRK               ; 00 
f2ad: ORA  $02          ; 05 02
f2af: .byte $62   ;?
f2b0: BRK               ; 00 
f2b1: STY  $02          ; 84 02
f2b3: .byte $02   ;?
f2b4: BRK               ; 00 
f2b5: ORA  $02          ; 05 02
f2b7: STZ  $00          ; 64 00
f2b9: ORA  ($00,X)      ; 01 00
f2bb: ORA  ($00,X)      ; 01 00
f2bd: .byte $02   ;?
f2be: .byte $ff   ;?
f2bf: .byte $a7   ;?
f2c0: CMP  ($81),Y      ; d1 81
f2c2: BRK               ; 00 
f2c3: ORA  ($00,X)      ; 01 00
f2c5: .byte $02   ;?
f2c6: BRK               ; 00 
f2c7: ROL  $01d2        ; 3e d2 01
f2ca: BRK               ; 00 
f2cb: .byte $07   ;?
f2cc: BRK               ; 00 
f2cd: .byte $02   ;?
f2ce: .byte $ff   ;?
f2cf: CMP  $d1          ; d5 d1
f2d1: STA  ($00,X)      ; 81 00
f2d3: .byte $07   ;?
f2d4: BRK               ; 00 
f2d5: .byte $02   ;?
f2d6: BRK               ; 00 
f2d7: STX  $d2,X        ; 86 d2
f2d9: ORA  ($00,X)      ; 01 00
f2db: .byte $02   ;?
f2dc: BRK               ; 00 
f2dd: .byte $02   ;?
f2de: .byte $ff   ;?
f2df: ORA  ($d0)        ; 12 d0
f2e1: STA  ($00,X)      ; 81 00
f2e3: .byte $02   ;?
f2e4: BRK               ; 00 
f2e5: .byte $02   ;?
f2e6: .byte $02   ;?
f2e7: ROR  $12,X        ; 76 12
f2e9: .byte $82   ;?
f2ea: BRK               ; 00 
f2eb: .byte $02   ;?
f2ec: BRK               ; 00 
f2ed: .byte $02   ;?
f2ee: .byte $02   ;?
f2ef: BVS  $f2f1        ; 70 00
f2f1: .byte $83   ;?
f2f2: BRK               ; 00 
f2f3: .byte $02   ;?
f2f4: BRK               ; 00 
f2f5: .byte $02   ;?
f2f6: .byte $02   ;?
f2f7: ADC  ($00)        ; 72 00
f2f9: STY  $00          ; 84 00
f2fb: .byte $02   ;?
f2fc: BRK               ; 00 
f2fd: .byte $02   ;?
f2fe: .byte $02   ;?
f2ff: STZ  $00          ; 74 00
f301: ORA  ($01,X)      ; 01 01
f303: .byte $02   ;?
f304: BRK               ; 00 
f305: .byte $02   ;?
f306: .byte $ff   ;?
f307: .byte $bf   ;?
f308: BNE  $f28b        ; d0 81
f30a: ORA  ($02,X)      ; 01 02
f30c: BRK               ; 00 
f30d: .byte $02   ;?
f30e: .byte $02   ;?
f30f: ROR  $12,X        ; 76 12
f311: .byte $82   ;?
f312: ORA  ($02,X)      ; 01 02
f314: BRK               ; 00 
f315: .byte $02   ;?
f316: .byte $02   ;?
f317: BVS  $f319        ; 70 00
f319: .byte $83   ;?
f31a: ORA  ($02,X)      ; 01 02
f31c: BRK               ; 00 
f31d: .byte $02   ;?
f31e: .byte $02   ;?
f31f: ADC  ($00)        ; 72 00
f321: STY  $01          ; 84 01
f323: .byte $02   ;?
f324: BRK               ; 00 
f325: .byte $02   ;?
f326: .byte $02   ;?
f327: STZ  $00          ; 74 00
f329: ORA  ($02,X)      ; 01 02
f32b: .byte $02   ;?
f32c: BRK               ; 00 
f32d: .byte $02   ;?
f32e: .byte $ff   ;?
f32f: CMP  $81d0,X      ; cd d0 81
f332: .byte $02   ;?
f333: .byte $02   ;?
f334: BRK               ; 00 
f335: .byte $02   ;?
f336: .byte $02   ;?
f337: .byte $78   ;?
f338: ORA  ($82)        ; 12 82
f33a: .byte $02   ;?
f33b: .byte $02   ;?
f33c: BRK               ; 00 
f33d: .byte $02   ;?
f33e: .byte $02   ;?
f33f: BVS  $f341        ; 70 00
f341: .byte $83   ;?
f342: .byte $02   ;?
f343: .byte $02   ;?
f344: BRK               ; 00 
f345: .byte $02   ;?
f346: .byte $02   ;?
f347: ADC  ($00)        ; 72 00
f349: STY  $02          ; 84 02
f34b: .byte $02   ;?
f34c: BRK               ; 00 
f34d: .byte $02   ;?
f34e: .byte $02   ;?
f34f: STZ  $00          ; 74 00
f351: ORA  ($00,X)      ; 01 00
f353: ORA  ($00,X)      ; 01 00
f355: .byte $0b   ;?
f356: .byte $ff   ;?
f357: STA  $d1          ; 85 d1
f359: STA  ($00,X)      ; 81 00
f35b: ORA  ($00,X)      ; 01 00
f35d: .byte $0b   ;?
f35e: BRK               ; 00 
f35f: AND  $01d2,X      ; 2d d2 01
f362: BRK               ; 00 
f363: .byte $02   ;?
f364: BRK               ; 00 
f365: .byte $0b   ;?
f366: .byte $ff   ;?
f367: .byte $79   ;?
f368: BNE  $f2eb        ; d0 81
f36a: BRK               ; 00 
f36b: .byte $02   ;?
f36c: BRK               ; 00 
f36d: .byte $0b   ;?
f36e: .byte $02   ;?
f36f: PLY               ; 7a 
f370: ORA  ($82)        ; 12 82
f372: BRK               ; 00 
f373: .byte $02   ;?
f374: BRK               ; 00 
f375: .byte $0b   ;?
f376: .byte $02   ;?
f377: .byte $78   ;?
f378: BRK               ; 00 
f379: .byte $83   ;?
f37a: BRK               ; 00 
f37b: .byte $02   ;?
f37c: BRK               ; 00 
f37d: .byte $0b   ;?
f37e: .byte $02   ;?
f37f: PLY               ; 7a 
f380: BRK               ; 00 
f381: STY  $00          ; 84 00
f383: .byte $02   ;?
f384: BRK               ; 00 
f385: .byte $0b   ;?
f386: .byte $02   ;?
f387: JMP  ($0100,X)    ; 7c 00 01
f38a: BRK               ; 00 
f38b: ORA  ($00,X)      ; 01 00
f38d: ASL  $90ff        ; 0e ff 90
f390: CMP  ($81),Y      ; d1 81
f392: BRK               ; 00 
f393: ORA  ($00,X)      ; 01 00
f395: ASL  $5600        ; 0e 00 56
f398: CMP  ($01)        ; d2 01
f39a: BRK               ; 00 
f39b: .byte $02   ;?
f39c: BRK               ; 00 
f39d: ASL  $87ff        ; 0e ff 87
f3a0: BNE  $f323        ; d0 81
f3a2: BRK               ; 00 
f3a3: .byte $02   ;?
f3a4: BRK               ; 00 
f3a5: ASL  $7202        ; 0e 02 72
f3a8: ORA  ($82)        ; 12 82
f3aa: BRK               ; 00 
f3ab: .byte $02   ;?
f3ac: BRK               ; 00 
f3ad: ASL  $8002        ; 0e 02 80
f3b0: BRK               ; 00 
f3b1: .byte $83   ;?
f3b2: BRK               ; 00 
f3b3: .byte $02   ;?
f3b4: BRK               ; 00 
f3b5: ASL  $8202        ; 0e 02 82
f3b8: BRK               ; 00 
f3b9: STY  $00          ; 84 00
f3bb: .byte $02   ;?
f3bc: BRK               ; 00 
f3bd: ASL  $8402        ; 0e 02 84
f3c0: BRK               ; 00 
f3c1: ORA  ($01,X)      ; 01 01
f3c3: .byte $02   ;?
f3c4: BRK               ; 00 
f3c5: ASL  $87ff        ; 0e ff 87
f3c8: BNE  $f34b        ; d0 81
f3ca: ORA  ($02,X)      ; 01 02
f3cc: BRK               ; 00 
f3cd: ASL  $7202        ; 0e 02 72
f3d0: ORA  ($82)        ; 12 82
f3d2: ORA  ($02,X)      ; 01 02
f3d4: BRK               ; 00 
f3d5: ASL  $8002        ; 0e 02 80
f3d8: BRK               ; 00 
f3d9: .byte $83   ;?
f3da: ORA  ($02,X)      ; 01 02
f3dc: BRK               ; 00 
f3dd: ASL  $8202        ; 0e 02 82
f3e0: BRK               ; 00 
f3e1: STY  $01          ; 84 01
f3e3: .byte $02   ;?
f3e4: BRK               ; 00 
f3e5: ASL  $8402        ; 0e 02 84
f3e8: BRK               ; 00 
f3e9: ORA  ($02,X)      ; 01 02
f3eb: .byte $02   ;?
f3ec: BRK               ; 00 
f3ed: ASL  $95ff        ; 0e ff 95
f3f0: BNE  $f373        ; d0 81
f3f2: .byte $02   ;?
f3f3: .byte $02   ;?
f3f4: BRK               ; 00 
f3f5: ASL  $7402        ; 0e 02 74
f3f8: ORA  ($82)        ; 12 82
f3fa: .byte $02   ;?
f3fb: .byte $02   ;?
f3fc: BRK               ; 00 
f3fd: ASL  $8002        ; 0e 02 80
f400: BRK               ; 00 
f401: .byte $83   ;?
f402: .byte $02   ;?
f403: .byte $02   ;?
f404: BRK               ; 00 
f405: ASL  $8202        ; 0e 02 82
f408: BRK               ; 00 
f409: STY  $02          ; 84 02
f40b: .byte $02   ;?
f40c: BRK               ; 00 
f40d: ASL  $8402        ; 0e 02 84
f410: BRK               ; 00 
f411: ORA  ($00,X)      ; 01 00
f413: BRK               ; 00 
f414: BRK               ; 00 
f415: BPL  $f416        ; 10 ff
f417: SBC  #$d1         ; e9 d1
f419: STA  ($00,X)      ; 81 00
f41b: BRK               ; 00 
f41c: BRK               ; 00 
f41d: BPL  $f41f        ; 10 00
f41f: SBC  $82d1,X      ; fd d1 82
f422: BRK               ; 00 
f423: BRK               ; 00 
f424: BRK               ; 00 
f425: BPL  $f428        ; 10 01
f427: CMP  $83f5,X      ; dd f5 83
f42a: BRK               ; 00 
f42b: BRK               ; 00 
f42c: BRK               ; 00 
f42d: BPL  $f430        ; 10 01
f42f: DEC  $84f5        ; de f5 84
f432: BRK               ; 00 
f433: BRK               ; 00 
f434: BRK               ; 00 
f435: BPL  $f438        ; 10 01
f437: CMP  $01f5,X      ; dd f5 01
f43a: BRK               ; 00 
f43b: ORA  ($00,X)      ; 01 00
f43d: ORA  ($ff),Y      ; 11 ff
f43f: LDX  $81d1        ; be d1 81
f442: BRK               ; 00 
f443: ORA  ($00,X)      ; 01 00
f445: ORA  ($00),Y      ; 11 00
f447: ROR  $01d2        ; 6e d2 01
f44a: BRK               ; 00 
f44b: .byte $02   ;?
f44c: BRK               ; 00 
f44d: ORA  ($ff),Y      ; 11 ff
f44f: DEA               ; 3a 
f450: BNE  $f3d3        ; d0 81
f452: BRK               ; 00 
f453: .byte $02   ;?
f454: BRK               ; 00 
f455: ORA  ($02),Y      ; 11 02
f457: ROR  $8212        ; 6e 12 82
f45a: BRK               ; 00 
f45b: .byte $02   ;?
f45c: BRK               ; 00 
f45d: ORA  ($02),Y      ; 11 02
f45f: PLA               ; 68 
f460: BRK               ; 00 
f461: .byte $83   ;?
f462: BRK               ; 00 
f463: .byte $02   ;?
f464: BRK               ; 00 
f465: ORA  ($02),Y      ; 11 02
f467: ROR               ; 6a 
f468: BRK               ; 00 
f469: STY  $00          ; 84 00
f46b: .byte $02   ;?
f46c: BRK               ; 00 
f46d: ORA  ($02),Y      ; 11 02
f46f: JMP  ($0100)      ; 6c 00 01
f472: ORA  ($02,X)      ; 01 02
f474: BRK               ; 00 
f475: ORA  ($ff),Y      ; 11 ff
f477: .byte $db   ;?
f478: BNE  $f3fb        ; d0 81
f47a: ORA  ($02,X)      ; 01 02
f47c: BRK               ; 00 
f47d: ORA  ($02),Y      ; 11 02
f47f: ROR  $8212        ; 6e 12 82
f482: ORA  ($02,X)      ; 01 02
f484: BRK               ; 00 
f485: ORA  ($02),Y      ; 11 02
f487: PLA               ; 68 
f488: BRK               ; 00 
f489: .byte $83   ;?
f48a: ORA  ($02,X)      ; 01 02
f48c: BRK               ; 00 
f48d: ORA  ($02),Y      ; 11 02
f48f: ROR               ; 6a 
f490: BRK               ; 00 
f491: STY  $01          ; 84 01
f493: .byte $02   ;?
f494: BRK               ; 00 
f495: ORA  ($02),Y      ; 11 02
f497: JMP  ($0100)      ; 6c 00 01
f49a: .byte $02   ;?
f49b: .byte $02   ;?
f49c: BRK               ; 00 
f49d: ORA  ($ff),Y      ; 11 ff
f49f: SBC  #$d0         ; e9 d0
f4a1: STA  ($02,X)      ; 81 02
f4a3: .byte $02   ;?
f4a4: BRK               ; 00 
f4a5: ORA  ($02),Y      ; 11 02
f4a7: BVS  $f4bb        ; 70 12
f4a9: .byte $82   ;?
f4aa: .byte $02   ;?
f4ab: .byte $02   ;?
f4ac: BRK               ; 00 
f4ad: ORA  ($02),Y      ; 11 02
f4af: PLA               ; 68 
f4b0: BRK               ; 00 
f4b1: .byte $83   ;?
f4b2: .byte $02   ;?
f4b3: .byte $02   ;?
f4b4: BRK               ; 00 
f4b5: ORA  ($02),Y      ; 11 02
f4b7: ROR               ; 6a 
f4b8: BRK               ; 00 
f4b9: STY  $02          ; 84 02
f4bb: .byte $02   ;?
f4bc: BRK               ; 00 
f4bd: ORA  ($02),Y      ; 11 02
f4bf: JMP  ($0100)      ; 6c 00 01
f4c2: BRK               ; 00 
f4c3: .byte $03   ;?
f4c4: BRK               ; 00 
f4c5: ORA  $ff          ; 05 ff
f4c7: CLC               ; 18 
f4c8: CMP  ($81),Y      ; d1 81
f4ca: BRK               ; 00 
f4cb: .byte $03   ;?
f4cc: BRK               ; 00 
f4cd: ORA  $01          ; 05 01
f4cf: .byte $67   ;?
f4d0: .byte $02   ;?
f4d1: .byte $82   ;?
f4d2: BRK               ; 00 
f4d3: .byte $03   ;?
f4d4: BRK               ; 00 
f4d5: ORA  $01          ; 05 01
f4d7: PLA               ; 68 
f4d8: .byte $02   ;?
f4d9: .byte $83   ;?
f4da: BRK               ; 00 
f4db: .byte $03   ;?
f4dc: BRK               ; 00 
f4dd: ORA  $01          ; 05 01
f4df: ADC  #$02         ; 69 02
f4e1: STY  $00          ; 84 00
f4e3: .byte $03   ;?
f4e4: BRK               ; 00 
f4e5: ORA  $01          ; 05 01
f4e7: ROR               ; 6a 
f4e8: .byte $02   ;?
f4e9: ORA  ($00,X)      ; 01 00
f4eb: ORA  $00          ; 05 00
f4ed: ORA  $ff          ; 05 ff
f4ef: JSR  $81d1        ; 20 d1 81
f4f2: BRK               ; 00 
f4f3: ORA  $00          ; 05 00
f4f5: ORA  $01          ; 05 01
f4f7: .byte $6b   ;?
f4f8: .byte $02   ;?
f4f9: .byte $82   ;?
f4fa: BRK               ; 00 
f4fb: ORA  $00          ; 05 00
f4fd: ORA  $01          ; 05 01
f4ff: JMP  ($8302)      ; 6c 02 83
f502: BRK               ; 00 
f503: ORA  $00          ; 05 00
f505: ORA  $01          ; 05 01
f507: ADC  $8402,X      ; 6d 02 84
f50a: BRK               ; 00 
f50b: ORA  $00          ; 05 00
f50d: ORA  $01          ; 05 01
f50f: ROR  $8502        ; 6e 02 85
f512: BRK               ; 00 
f513: BRK               ; 00 
f514: BRK               ; 00 
f515: ORA  ($00,X)      ; 01 00
f517: .byte $99   ;?
f518: CMP  ($ff)        ; d2 ff
f51a: ORA  ($00,X)      ; 01 00
f51c: ORA  ($01,X)      ; 01 01
f51e: BRK               ; 00 
f51f: .byte $ff   ;?
f520: SED               ; f8 
f521: CMP  ($81)        ; d2 81
f523: BRK               ; 00 
f524: ORA  ($01,X)      ; 01 01
f526: BRK               ; 00 
f527: BRK               ; 00 
f528: .byte $d9   ;?
f529: CMP  ($82)        ; d2 82
f52b: BRK               ; 00 
f52c: ORA  ($01,X)      ; 01 01
f52e: BRK               ; 00 
f52f: .byte $03   ;?
f530: SED               ; f8 
f531: SBC  $83          ; f5 83
f533: BRK               ; 00 
f534: ORA  ($01,X)      ; 01 01
f536: BRK               ; 00 
f537: .byte $03   ;?
f538: TRB  ($84f6)      ; 1c f6 84
f53b: BRK               ; 00 
f53c: ORA  ($01,X)      ; 01 01
f53e: BRK               ; 00 
f53f: .byte $03   ;?
f540: .byte $df   ;?
f541: SBC  $01          ; f5 01
f543: BRK               ; 00 
f544: ORA  ($81,X)      ; 01 81
f546: BRK               ; 00 
f547: .byte $ff   ;?
f548: .byte $ff   ;?
f549: CMP  ($81)        ; d2 81
f54b: BRK               ; 00 
f54c: ORA  ($81,X)      ; 01 81
f54e: BRK               ; 00 
f54f: BRK               ; 00 
f550: .byte $f3   ;?
f551: CMP  ($82)        ; d2 82
f553: BRK               ; 00 
f554: ORA  ($81,X)      ; 01 81
f556: BRK               ; 00 
f557: .byte $03   ;?
f558: SED               ; f8 
f559: SBC  $83          ; f5 83
f55b: BRK               ; 00 
f55c: ORA  ($81,X)      ; 01 81
f55e: BRK               ; 00 
f55f: .byte $03   ;?
f560: TRB  ($84f6)      ; 1c f6 84
f563: BRK               ; 00 
f564: ORA  ($81,X)      ; 01 81
f566: BRK               ; 00 
f567: .byte $03   ;?
f568: .byte $df   ;?
f569: SBC  $ff          ; f5 ff
f56b: ASL               ; 0a 
f56c: BRK               ; 00 
f56d: BRK               ; 00 
f56e: .byte $03   ;?
f56f: BRK               ; 00 
f570: BRK               ; 00 
f571: ROL  $01d3        ; 2e d3 01
f574: ORA  ($01,X)      ; 01 01
f576: .byte $03   ;?
f577: BRK               ; 00 
f578: .byte $02   ;?
f579: JMP  ($0112,X)    ; 7c 12 01
f57c: .byte $02   ;?
f57d: ORA  ($03,X)      ; 01 03
f57f: BRK               ; 00 
f580: .byte $02   ;?
f581: ROR  $0912        ; 7e 12 09
f584: .byte $03   ;?
f585: .byte $02   ;?
f586: .byte $03   ;?
f587: BRK               ; 00 
f588: .byte $ff   ;?
f589: ROL  $09d3        ; 3e d3 09
f58c: TSB  $02          ; 04 02
f58e: .byte $03   ;?
f58f: BRK               ; 00 
f590: .byte $ff   ;?
f591: .byte $4b   ;?
f592: .byte $d3   ;?
f593: ORA  ($05,X)      ; 01 05
f595: ORA  ($03,X)      ; 01 03
f597: BRK               ; 00 
f598: .byte $23   ;?
f599: STA  ($12,X)      ; 81 12
f59b: ORA  #$06         ; 09 06
f59d: .byte $02   ;?
f59e: .byte $03   ;?
f59f: BRK               ; 00 
f5a0: .byte $ff   ;?
f5a1: STZ  $d4          ; 74 d4
f5a3: ORA  ($07,X)      ; 01 07
f5a5: ORA  ($03,X)      ; 01 03
f5a7: BRK               ; 00 
f5a8: AND  ($a4,X)      ; 21 a4
f5aa: ORA  ($01)        ; 12 01
f5ac: PHP               ; 08 
f5ad: ORA  ($03,X)      ; 01 03
f5af: BRK               ; 00 
f5b0: .byte $02   ;?
f5b1: CMP  $12          ; c5 12
f5b3: ORA  #$08         ; 09 08
f5b5: .byte $02   ;?
f5b6: .byte $03   ;?
f5b7: BRK               ; 00 
f5b8: .byte $ff   ;?
f5b9: AND  ($d3),Y      ; 31 d3
f5bb: .byte $ff   ;?
f5bc: SBC  #$dd         ; e9 dd
f5be: .byte $17   ;?
f5bf: .byte $22   ;?
f5c0: EOR  ($e1)        ; 52 e1
f5c2: LDX  $bb1e        ; ae 1e bb
f5c5: CPX  $45          ; e4 45
f5c7: .byte $1b   ;?
f5c8: .byte $23   ;?
f5c9: INX               ; e8 
f5ca: CMP  $8c17,X      ; dd 17 8c
f5cd: SBC               ; eb 
f5ce: STZ  $14          ; 74 14
f5d0: SBC  $ee          ; f5 ee
f5d2: .byte $0b   ;?
f5d3: ORA  ($5d),Y      ; 11 5d
f5d5: SBC  ($a3)        ; f2 a3
f5d7: ORA  $f5c6,X      ; 0d c6 f5
f5da: DEA               ; 3a 
f5db: ASL               ; 0a 
f5dc: BRK               ; 00 
f5dd: ORA  ($02,X)      ; 01 02
f5df: BIT  ($000f)      ; 3c 0f 00
f5e2: SED               ; f8 
f5e3: SBC  $fc          ; f5 fc
f5e5: SBC  $00          ; f5 00
f5e7: INC  $04,X        ; f6 04
f5e9: INC  $08,X        ; f6 08
f5eb: INC  $0c,X        ; f6 0c
f5ed: INC  $10,X        ; f6 10
f5ef: INC  $14,X        ; f6 14
f5f1: INC  $18,X        ; f6 18
f5f3: INC  $1c,X        ; f6 1c
f5f5: INC  $20,X        ; f6 20
f5f7: INC  $40,X        ; f6 40
f5f9: .byte $1f   ;?
f5fa: BRK               ; 00 
f5fb: BRK               ; 00 
f5fc: ORA  ($2b),Y      ; 11 2b
f5fe: BRK               ; 00 
f5ff: BRK               ; 00 
f600: BRA  $f640        ; 80 3e
f602: BRK               ; 00 
f603: BRK               ; 00 
f604: .byte $22   ;?
f605: LSR  $00,X        ; 56 00
f607: BRK               ; 00 
f608: CPY  #$5d         ; c0 5d
f60a: BRK               ; 00 
f60b: BRK               ; 00 
f60c: BRK               ; 00 
f60d: ADC  $0000,X      ; 7d 00 00
f610: TSB  $ac          ; 44 ac
f612: BRK               ; 00 
f613: BRK               ; 00 
f614: BRA  $f5d1        ; 80 bb
f616: BRK               ; 00 
f617: BRK               ; 00 
f618: DEY               ; 88 
f619: CLI               ; 58 
f61a: ORA  ($00,X)      ; 01 00
f61c: BRK               ; 00 
f61d: .byte $77   ;?
f61e: ORA  ($00,X)      ; 01 00
f620: BRK               ; 00 
f621: INC  $0002        ; ee 02 00
f624: JSR  $0000        ; 20 00 00
f627: RTI               ; 40 
f628: BRK               ; 00 
f629: .byte $02   ;?
f62a: RTS               ; 60 
f62b: BRK               ; 00 
f62c: TSB  $80          ; 04 80
f62e: BRK               ; 00 
f62f: ORA  $b0          ; 05 b0
f631: BRK               ; 00 
f632: ASL  $b4,X        ; 06 b4
f634: BRK               ; 00 
f635: ASL  $c0,X        ; 06 c0
f637: BRK               ; 00 
f638: .byte $07   ;?
f639: RTS               ; 60 
f63a: ORA  ($08,X)      ; 01 08
f63c: STZ  $01          ; 64 01
f63e: PHP               ; 08 
f63f: BRA  $f642        ; 80 01
f641: ORA  #$00         ; 09 00
f643: .byte $03   ;?
f644: ASL               ; 0a 
f645: BMI  $f647        ; 30 00
f647: BRK               ; 00 
f648: RTS               ; 60 
f649: BRK               ; 00 
f64a: .byte $02   ;?
f64b: BCC  $f64d        ; 90 00
f64d: TSB  $c0          ; 04 c0
f64f: BRK               ; 00 
f650: ORA  $08          ; 05 08
f652: ORA  ($06,X)      ; 01 06
f654: ASL  $0601        ; 0e 01 06
f657: JSR  $0701        ; 20 01 07
f65a: BPL  $f65e        ; 10 02
f65c: PHP               ; 08 
f65d: ASL  $02,X        ; 16 02
f65f: PHP               ; 08 
f660: RTI               ; 40 
f661: .byte $02   ;?
f662: ORA  #$20         ; 09 20
f664: BRK               ; 00 
f665: BIT  ($4000)      ; 2c 00 40
f668: BRK               ; 00 
f669: CLI               ; 58 
f66a: BRK               ; 00 
f66b: RTS               ; 60 
f66c: BRK               ; 00 
f66d: BRA  $f66f        ; 80 00
f66f: BCS  $f671        ; b0 00
f671: CPY  #$00         ; c0 00
f673: BIT  $00          ; 24 00
f675: BMI  $f677        ; 30 00
f677: TSB  $00          ; 44 00
f679: JMP  ($6400,X)    ; 5c 00 64
f67c: BRK               ; 00 
f67d: STY  $00          ; 84 00
f67f: LDY  $00,X        ; b4 00
f681: CPY  $00          ; c4 00
f683: TRB  ($2c00)      ; 1c 00 2c
f686: BRK               ; 00 
f687: BIT  ($5800)      ; 3c 00 58
f68a: BRK               ; 00 
f68b: JMP  ($7c00,X)    ; 5c 00 7c
f68e: BRK               ; 00 
f68f: BCS  $f691        ; b0 00
f691: LDY  ($3000)      ; bc 00 30
f694: BRK               ; 00 
f695: .byte $42   ;?
f696: BRK               ; 00 
f697: RTS               ; 60 
f698: BRK               ; 00 
f699: STY  $00          ; 84 00
f69b: BCC  $f69d        ; 90 00
f69d: CPY  #$00         ; c0 00
f69f: PHP               ; 08 
f6a0: ORA  ($20,X)      ; 01 20
f6a2: ORA  ($36,X)      ; 01 36
f6a4: BRK               ; 00 
f6a5: PHA               ; 48 
f6a6: BRK               ; 00 
f6a7: ROR  $00,X        ; 66 00
f6a9: TXA               ; 8a 
f6aa: BRK               ; 00 
f6ab: STX  $00,Y        ; 96 00
f6ad: DEC  $00,X        ; c6 00
f6af: ASL  $2601        ; 0e 01 26
f6b2: ORA  ($2a,X)      ; 01 2a
f6b4: BRK               ; 00 
f6b5: .byte $42   ;?
f6b6: BRK               ; 00 
f6b7: PHY               ; 5a 
f6b8: BRK               ; 00 
f6b9: STY  $00          ; 84 00
f6bb: TXA               ; 8a 
f6bc: BRK               ; 00 
f6bd: TSX               ; ba 
f6be: BRK               ; 00 
f6bf: PHP               ; 08 
f6c0: ORA  ($1a,X)      ; 01 1a
f6c2: ORA  ($11,X)      ; 01 11
f6c4: LSR  $0004        ; 5e 04 00
f6c7: .byte $02   ;?
f6c8: ORA  ($01)        ; 12 01
f6ca: BRK               ; 00 
f6cb: .byte $02   ;?
f6cc: .byte $ef   ;?
f6cd: .byte $02   ;?
f6ce: ORA  ($40,X)      ; 01 40
f6d0: ADC  ($05)        ; 72 05
f6d2: BRK               ; 00 
f6d3: INA               ; 1a 
f6d4: BRK               ; 00 
f6d5: .byte $02   ;?
f6d6: ORA  ($02,X)      ; 01 02
f6d8: .byte $03   ;?
f6d9: ORA  ($05,X)      ; 01 05
f6db: BRK               ; 00 
f6dc: RTI               ; 40 
f6dd: .byte $1f   ;?
f6de: BRK               ; 00 
f6df: BRK               ; 00 
f6e0: RTI               ; 40 
f6e1: .byte $1f   ;?
f6e2: BRK               ; 00 
f6e3: BRK               ; 00 
f6e4: BRK               ; 00 
f6e5: BRK               ; 00 
f6e6: BRK               ; 00 
f6e7: BRK               ; 00 
f6e8: BRA  $f728        ; 80 3e
f6ea: BRK               ; 00 
f6eb: BRK               ; 00 
f6ec: BRA  $f72c        ; 80 3e
f6ee: BRK               ; 00 
f6ef: BRK               ; 00 
f6f0: BRK               ; 00 
f6f1: BRK               ; 00 
f6f2: BRK               ; 00 
f6f3: BRK               ; 00 
f6f4: BRK               ; 00 
f6f5: ADC  $0000,X      ; 7d 00 00
f6f8: BRK               ; 00 
f6f9: ADC  $0000,X      ; 7d 00 00
f6fc: BRK               ; 00 
f6fd: BRK               ; 00 
f6fe: BRK               ; 00 
f6ff: BRK               ; 00 
f700: TSB  $ac          ; 44 ac
f702: BRK               ; 00 
f703: BRK               ; 00 
f704: TSB  $ac          ; 44 ac
f706: BRK               ; 00 
f707: BRK               ; 00 
f708: BRK               ; 00 
f709: BRK               ; 00 
f70a: BRK               ; 00 
f70b: BRK               ; 00 
f70c: BRA  $f6c9        ; 80 bb
f70e: BRK               ; 00 
f70f: BRK               ; 00 
f710: BRA  $f6cd        ; 80 bb
f712: BRK               ; 00 
f713: BRK               ; 00 
f714: BRK               ; 00 
f715: BRK               ; 00 
f716: BRK               ; 00 
f717: BRK               ; 00 
f718: .byte $07   ;?
f719: BRK               ; 00 
f71a: RTI               ; 40 
f71b: .byte $1f   ;?
f71c: BRK               ; 00 
f71d: BRK               ; 00 
f71e: RTI               ; 40 
f71f: .byte $1f   ;?
f720: BRK               ; 00 
f721: BRK               ; 00 
f722: BRK               ; 00 
f723: BRK               ; 00 
f724: BRK               ; 00 
f725: BRK               ; 00 
f726: BRA  $f766        ; 80 3e
f728: BRK               ; 00 
f729: BRK               ; 00 
f72a: BRA  $f76a        ; 80 3e
f72c: BRK               ; 00 
f72d: BRK               ; 00 
f72e: BRK               ; 00 
f72f: BRK               ; 00 
f730: BRK               ; 00 
f731: BRK               ; 00 
f732: BRK               ; 00 
f733: ADC  $0000,X      ; 7d 00 00
f736: BRK               ; 00 
f737: ADC  $0000,X      ; 7d 00 00
f73a: BRK               ; 00 
f73b: BRK               ; 00 
f73c: BRK               ; 00 
f73d: BRK               ; 00 
f73e: TSB  $ac          ; 44 ac
f740: BRK               ; 00 
f741: BRK               ; 00 
f742: TSB  $ac          ; 44 ac
f744: BRK               ; 00 
f745: BRK               ; 00 
f746: BRK               ; 00 
f747: BRK               ; 00 
f748: BRK               ; 00 
f749: BRK               ; 00 
f74a: BRA  $f707        ; 80 bb
f74c: BRK               ; 00 
f74d: BRK               ; 00 
f74e: BRA  $f70b        ; 80 bb
f750: BRK               ; 00 
f751: BRK               ; 00 
f752: BRK               ; 00 
f753: BRK               ; 00 
f754: BRK               ; 00 
f755: BRK               ; 00 
f756: BRK               ; 00 
f757: .byte $77   ;?
f758: BRK               ; 00 
f759: BRK               ; 00 
f75a: BRK               ; 00 
f75b: .byte $77   ;?
f75c: BRK               ; 00 
f75d: BRK               ; 00 
f75e: BRK               ; 00 
f75f: BRK               ; 00 
f760: BRK               ; 00 
f761: BRK               ; 00 
f762: BRK               ; 00 
f763: INC  $0000        ; ee 00 00
f766: BRK               ; 00 
f767: INC  $0000        ; ee 00 00
f76a: BRK               ; 00 
f76b: BRK               ; 00 
f76c: BRK               ; 00 
f76d: BRK               ; 00 
f76e: TSB  ($0600)      ; 0c 00 06
f771: BRK               ; 00 
f772: BRK               ; 00 
f773: ASL  $04,X        ; 06 04
f775: TSB  $02          ; 04 02
f777: .byte $02   ;?
f778: .byte $02   ;?
f779: TSB  ($0300)      ; 0c 00 03
f77c: BRK               ; 00 
f77d: BRK               ; 00 
f77e: .byte $03   ;?
f77f: .byte $02   ;?
f780: .byte $02   ;?
f781: BRK               ; 00 
f782: RTI               ; 40 
f783: BRK               ; 00 
f784: BRK               ; 00 
f785: BRK               ; 00 
f786: BRK               ; 00 
f787: BRK               ; 00 
f788: BRK               ; 00 
f789: BRK               ; 00 
f78a: BRK               ; 00 
f78b: .byte $03   ;?
f78c: BRK               ; 00 
f78d: TSB  $03          ; 04 03
f78f: .byte $0b   ;?
f790: .byte $02   ;?
f791: BRK               ; 00 
f792: ASL               ; 0a 
f793: BRK               ; 00 
f794: ORA  ($00,X)      ; 01 00
f796: BRK               ; 00 
f797: BRK               ; 00 
f798: ORA  #$00         ; 09 00
f79a: PHP               ; 08 
f79b: .byte $07   ;?
f79c: ASL  $0006        ; 0e 06 00
f79f: ORA  $0500,X      ; 0d 00 05
f7a2: BRK               ; 00 
f7a3: BRK               ; 00 
f7a4: BRK               ; 00 
f7a5: TSB  ($e8d4)      ; 0c d4 e8
f7a8: .byte $db   ;?
f7a9: INX               ; e8 
f7aa: .byte $df   ;?
f7ab: INX               ; e8 
f7ac: .byte $e3   ;?
f7ad: INX               ; e8 
f7ae: .byte $f7   ;?
f7af: INX               ; e8 
f7b0: AND  ($e9,X)      ; 21 e9
f7b2: BIT  ($37e9)      ; 2c e9 37
f7b5: SBC  #$63         ; e9 63
f7b7: SBC  #$8a         ; e9 8a
f7b9: SBC  #$8b         ; e9 8b
f7bb: SBC  #$8c         ; e9 8c
f7bd: SBC  #$8d         ; e9 8d
f7bf: SBC  #$8e         ; e9 8e
f7c1: SBC  #$8f         ; e9 8f
f7c3: SBC  #$96         ; e9 96
f7c5: SBC  #$77         ; e9 77
f7c7: NOP               ; ea 
f7c8: .byte $78   ;?
f7c9: NOP               ; ea 
f7ca: .byte $13   ;?
f7cb: NOP               ; ea 
f7cc: PHA               ; 48 
f7cd: NOP               ; ea 
f7ce: RTS               ; 60 
f7cf: NOP               ; ea 
f7d0: ADC  ($ea)        ; 72 ea
f7d2: ROR  $d6ea        ; 7e ea d6
f7d5: SBC  #$00         ; e9 00
f7d7: BRK               ; 00 
f7d8: BRK               ; 00 
f7d9: BVS  $f7db        ; 70 00
f7db: ORA  ($02,X)      ; 01 02
f7dd: .byte $03   ;?
f7de: TSB  $05          ; 04 05
f7e0: ASL  $07,X        ; 06 07
f7e2: PHP               ; 08 
f7e3: ORA  #$0a         ; 09 0a
f7e5: .byte $0b   ;?
f7e6: TSB  ($0e0d)      ; 0c 0d 0e
f7e9: .byte $0f   ;?
f7ea: BRK               ; 00 
f7eb: DEC  $0000        ; ce 00 00
f7ee: BRK               ; 00 
f7ef: ORA  ($00,X)      ; 01 00
f7f1: BRK               ; 00 
f7f2: BRK               ; 00 
f7f3: LDX  $00,Y        ; b6 00
f7f5: BRK               ; 00 
f7f6: BRK               ; 00 
f7f7: ORA  ($00,X)      ; 01 00
f7f9: BRK               ; 00 
f7fa: BRK               ; 00 
f7fb: .byte $e2   ;?
f7fc: BRK               ; 00 
f7fd: BRK               ; 00 
f7fe: BRK               ; 00 
f7ff: ORA  ($00,X)      ; 01 00
f801: BRK               ; 00 
f802: BRK               ; 00 
f803: .byte $c7   ;?
f804: BRK               ; 00 
f805: CPX  $80          ; f4 80
f807: ORA  ($00,X)      ; 01 00
f809: .byte $e2   ;?
f80a: BRK               ; 00 
f80b: .byte $c7   ;?
f80c: BRK               ; 00 
f80d: BRK               ; 00 
f80e: BRA  $f811        ; 80 01
f810: BRK               ; 00 
f811: BRK               ; 00 
f812: AND  ($01)        ; 32 01
f814: .byte $03   ;?
f815: BRK               ; 00 
f816: ASL  $03,X        ; 06 03
f818: BRK               ; 00 
f819: ORA  ($02,X)      ; 01 02
f81b: BRK               ; 00 
f81c: TSB  $00          ; 04 00
f81e: BRK               ; 00 
f81f: BRK               ; 00 
f820: BRK               ; 00 
f821: BRK               ; 00 
f822: BRK               ; 00 
f823: BRK               ; 00 
f824: BRK               ; 00 
f825: BRK               ; 00 
f826: BRK               ; 00 
f827: ORA  $05          ; 05 05
f829: ORA  $05          ; 05 05
f82b: ORA  $05          ; 05 05
f82d: BRK               ; 00 
f82e: BRK               ; 00 
f82f: BRK               ; 00 
f830: BRA  $f7b2        ; 80 80
f832: BRK               ; 00 
f833: JMP  ($0001,X)    ; 5c 01 00
f836: RTI               ; 40 
f837: BRK               ; 00 
f838: BRK               ; 00 
f839: BRK               ; 00 
f83a: BRK               ; 00 
f83b: BRK               ; 00 
f83c: BRK               ; 00 
f83d: BRK               ; 00 
f83e: BRK               ; 00 
f83f: .byte $03   ;?
f840: BRK               ; 00 
f841: RTI               ; 40 
f842: BRK               ; 00 
f843: BRK               ; 00 
f844: BRK               ; 00 
f845: BRK               ; 00 
f846: BRK               ; 00 
f847: BRK               ; 00 
f848: BRK               ; 00 
f849: BRK               ; 00 
f84a: .byte $03   ;?
f84b: BRK               ; 00 
f84c: RTI               ; 40 
f84d: BRK               ; 00 
f84e: BRK               ; 00 
f84f: BRK               ; 00 
f850: BRK               ; 00 
f851: BRK               ; 00 
f852: BRK               ; 00 
f853: BRK               ; 00 
f854: BRK               ; 00 
f855: .byte $03   ;?
f856: BRK               ; 00 
f857: RTI               ; 40 
f858: BRK               ; 00 
f859: BRK               ; 00 
f85a: BRK               ; 00 
f85b: BRK               ; 00 
f85c: BRK               ; 00 
f85d: BRK               ; 00 
f85e: BRK               ; 00 
f85f: BRK               ; 00 
f860: .byte $03   ;?
f861: BRK               ; 00 
f862: RTI               ; 40 
f863: BRK               ; 00 
f864: BRK               ; 00 
f865: BRK               ; 00 
f866: BRK               ; 00 
f867: BRK               ; 00 
f868: BRK               ; 00 
f869: BRK               ; 00 
f86a: BRK               ; 00 
f86b: .byte $03   ;?
f86c: BRK               ; 00 
f86d: RTI               ; 40 
f86e: BRK               ; 00 
f86f: BRK               ; 00 
f870: BRK               ; 00 
f871: BRK               ; 00 
f872: BRK               ; 00 
f873: BRK               ; 00 
f874: BRK               ; 00 
f875: BRK               ; 00 
f876: .byte $03   ;?
f877: BRK               ; 00 
f878: RTI               ; 40 
f879: BRK               ; 00 
f87a: BRK               ; 00 
f87b: BRK               ; 00 
f87c: BRK               ; 00 
f87d: BRK               ; 00 
f87e: BRK               ; 00 
f87f: BRK               ; 00 
f880: BRK               ; 00 
f881: .byte $03   ;?
f882: BRK               ; 00 
f883: RTI               ; 40 
f884: BRK               ; 00 
f885: BRK               ; 00 
f886: BRK               ; 00 
f887: BRK               ; 00 
f888: BRK               ; 00 
f889: BRK               ; 00 
f88a: BRK               ; 00 
f88b: BRK               ; 00 
f88c: .byte $03   ;?
f88d: BRK               ; 00 
f88e: RTI               ; 40 
f88f: BRK               ; 00 
f890: BRK               ; 00 
f891: BRK               ; 00 
f892: BRK               ; 00 
f893: BRK               ; 00 
f894: BRK               ; 00 
f895: BRK               ; 00 
f896: BRK               ; 00 
f897: .byte $03   ;?
f898: BRK               ; 00 
f899: RTI               ; 40 
f89a: BRK               ; 00 
f89b: BRK               ; 00 
f89c: BRK               ; 00 
f89d: BRK               ; 00 
f89e: BRK               ; 00 
f89f: BRK               ; 00 
f8a0: BRK               ; 00 
f8a1: BRK               ; 00 
f8a2: .byte $03   ;?
f8a3: BRK               ; 00 
f8a4: RTI               ; 40 
f8a5: BRK               ; 00 
f8a6: BRK               ; 00 
f8a7: BRK               ; 00 
f8a8: BRK               ; 00 
f8a9: BRK               ; 00 
f8aa: BRK               ; 00 
f8ab: BRK               ; 00 
f8ac: BRK               ; 00 
f8ad: .byte $03   ;?
f8ae: BRK               ; 00 
f8af: RTI               ; 40 
f8b0: BRK               ; 00 
f8b1: BRK               ; 00 
f8b2: BRK               ; 00 
f8b3: BRK               ; 00 
f8b4: BRK               ; 00 
f8b5: BRK               ; 00 
f8b6: BRK               ; 00 
f8b7: BRK               ; 00 
f8b8: .byte $03   ;?
f8b9: BRK               ; 00 
f8ba: RTI               ; 40 
f8bb: BRK               ; 00 
f8bc: BRK               ; 00 
f8bd: BRK               ; 00 
f8be: BRK               ; 00 
f8bf: BRK               ; 00 
f8c0: BRK               ; 00 
f8c1: BRK               ; 00 
f8c2: BRK               ; 00 
f8c3: .byte $03   ;?
f8c4: BRK               ; 00 
f8c5: RTI               ; 40 
f8c6: BRK               ; 00 
f8c7: BRK               ; 00 
f8c8: BRK               ; 00 
f8c9: BRK               ; 00 
f8ca: BRK               ; 00 
f8cb: BRK               ; 00 
f8cc: BRK               ; 00 
f8cd: BRK               ; 00 
f8ce: .byte $03   ;?
f8cf: BRK               ; 00 
f8d0: RTI               ; 40 
f8d1: BRK               ; 00 
f8d2: BRK               ; 00 
f8d3: BRK               ; 00 
f8d4: BRK               ; 00 
f8d5: BRK               ; 00 
f8d6: BRK               ; 00 
f8d7: BRK               ; 00 
f8d8: BRK               ; 00 
f8d9: .byte $03   ;?
f8da: BRK               ; 00 
f8db: RTI               ; 40 
f8dc: BRK               ; 00 
f8dd: BRK               ; 00 
f8de: BRK               ; 00 
f8df: BRK               ; 00 
f8e0: BRK               ; 00 
f8e1: BRK               ; 00 
f8e2: BRK               ; 00 
f8e3: BRK               ; 00 
f8e4: .byte $03   ;?
f8e5: BRK               ; 00 
f8e6: RTI               ; 40 
f8e7: BRK               ; 00 
f8e8: BRK               ; 00 
f8e9: BRK               ; 00 
f8ea: BRK               ; 00 
f8eb: BRK               ; 00 
f8ec: BRK               ; 00 
f8ed: BRK               ; 00 
f8ee: BRK               ; 00 
f8ef: .byte $03   ;?
f8f0: BRK               ; 00 
f8f1: RTI               ; 40 
f8f2: BRK               ; 00 
f8f3: BRK               ; 00 
f8f4: BRK               ; 00 
f8f5: BRK               ; 00 
f8f6: BRK               ; 00 
f8f7: BRK               ; 00 
f8f8: BRK               ; 00 
f8f9: BRK               ; 00 
f8fa: .byte $03   ;?
f8fb: BRK               ; 00 
f8fc: RTI               ; 40 
f8fd: BRK               ; 00 
f8fe: BRK               ; 00 
f8ff: BRK               ; 00 
f900: BRK               ; 00 
f901: BRK               ; 00 
f902: BRK               ; 00 
f903: BRK               ; 00 
f904: BRK               ; 00 
f905: .byte $03   ;?
f906: BRK               ; 00 
f907: RTI               ; 40 
f908: BRK               ; 00 
f909: BRK               ; 00 
f90a: BRK               ; 00 
f90b: BRK               ; 00 
f90c: BRK               ; 00 
f90d: BRK               ; 00 
f90e: BRK               ; 00 
f90f: BRK               ; 00 
f910: .byte $03   ;?
f911: BRK               ; 00 
f912: RTI               ; 40 
f913: BRK               ; 00 
f914: BRK               ; 00 
f915: BRK               ; 00 
f916: BRK               ; 00 
f917: BRK               ; 00 
f918: BRK               ; 00 
f919: BRK               ; 00 
f91a: BRK               ; 00 
f91b: .byte $03   ;?
f91c: BRK               ; 00 
f91d: RTI               ; 40 
f91e: BRK               ; 00 
f91f: BRK               ; 00 
f920: BRK               ; 00 
f921: BRK               ; 00 
f922: BRK               ; 00 
f923: BRK               ; 00 
f924: BRK               ; 00 
f925: BRK               ; 00 
f926: .byte $03   ;?
f927: BRK               ; 00 
f928: RTI               ; 40 
f929: BRK               ; 00 
f92a: BRK               ; 00 
f92b: BRK               ; 00 
f92c: BRK               ; 00 
f92d: BRK               ; 00 
f92e: BRK               ; 00 
f92f: BRK               ; 00 
f930: BRK               ; 00 
f931: .byte $03   ;?
f932: BRK               ; 00 
f933: RTI               ; 40 
f934: BRK               ; 00 
f935: BRK               ; 00 
f936: BRK               ; 00 
f937: BRK               ; 00 
f938: BRK               ; 00 
f939: BRK               ; 00 
f93a: BRK               ; 00 
f93b: BRK               ; 00 
f93c: .byte $03   ;?
f93d: BRK               ; 00 
f93e: RTI               ; 40 
f93f: BRK               ; 00 
f940: BRK               ; 00 
f941: BRK               ; 00 
f942: BRK               ; 00 
f943: BRK               ; 00 
f944: BRK               ; 00 
f945: BRK               ; 00 
f946: BRK               ; 00 
f947: .byte $03   ;?
f948: BRK               ; 00 
f949: BPL  $f94b        ; 10 00
f94b: RTI               ; 40 
f94c: BRK               ; 00 
f94d: BRK               ; 00 
f94e: BRK               ; 00 
f94f: BRK               ; 00 
f950: BRK               ; 00 
f951: BRK               ; 00 
f952: BRK               ; 00 
f953: BRK               ; 00 
f954: .byte $03   ;?
f955: BRK               ; 00 
f956: RTI               ; 40 
f957: BRK               ; 00 
f958: BRK               ; 00 
f959: BRK               ; 00 
f95a: BRK               ; 00 
f95b: BRK               ; 00 
f95c: BRK               ; 00 
f95d: BRK               ; 00 
f95e: BRK               ; 00 
f95f: .byte $03   ;?
f960: BRK               ; 00 
f961: RTI               ; 40 
f962: BRK               ; 00 
f963: BRK               ; 00 
f964: BRK               ; 00 
f965: BRK               ; 00 
f966: BRK               ; 00 
f967: BRK               ; 00 
f968: BRK               ; 00 
f969: BRK               ; 00 
f96a: .byte $03   ;?
f96b: BRK               ; 00 
f96c: RTI               ; 40 
f96d: BRK               ; 00 
f96e: BRK               ; 00 
f96f: BRK               ; 00 
f970: BRK               ; 00 
f971: BRK               ; 00 
f972: BRK               ; 00 
f973: BRK               ; 00 
f974: BRK               ; 00 
f975: .byte $03   ;?
f976: BRK               ; 00 
f977: RTI               ; 40 
f978: BRK               ; 00 
f979: BRK               ; 00 
f97a: BRK               ; 00 
f97b: BRK               ; 00 
f97c: BRK               ; 00 
f97d: BRK               ; 00 
f97e: BRK               ; 00 
f97f: BRK               ; 00 
f980: .byte $03   ;?
f981: BRK               ; 00 
f982: RTI               ; 40 
f983: BRK               ; 00 
f984: BRK               ; 00 
f985: BRK               ; 00 
f986: BRK               ; 00 
f987: BRK               ; 00 
f988: BRK               ; 00 
f989: BRK               ; 00 
f98a: BRK               ; 00 
f98b: .byte $03   ;?
f98c: BRK               ; 00 
f98d: RTI               ; 40 
f98e: BRK               ; 00 
f98f: BRK               ; 00 
f990: BRK               ; 00 
f991: BRK               ; 00 
f992: BRK               ; 00 
f993: BRK               ; 00 
f994: BRK               ; 00 
f995: BRK               ; 00 
f996: .byte $03   ;?
f997: BRK               ; 00 
f998: RTI               ; 40 
f999: BRK               ; 00 
f99a: BRK               ; 00 
f99b: BRK               ; 00 
f99c: BRK               ; 00 
f99d: BRK               ; 00 
f99e: BRK               ; 00 
f99f: BRK               ; 00 
f9a0: BRK               ; 00 
f9a1: .byte $03   ;?
f9a2: BRK               ; 00 
f9a3: RTI               ; 40 
f9a4: BRK               ; 00 
f9a5: BRK               ; 00 
f9a6: BRK               ; 00 
f9a7: BRK               ; 00 
f9a8: BRK               ; 00 
f9a9: BRK               ; 00 
f9aa: BRK               ; 00 
f9ab: BRK               ; 00 
f9ac: .byte $03   ;?
f9ad: BRK               ; 00 
f9ae: RTI               ; 40 
f9af: BRK               ; 00 
f9b0: BRK               ; 00 
f9b1: BRK               ; 00 
f9b2: BRK               ; 00 
f9b3: BRK               ; 00 
f9b4: BRK               ; 00 
f9b5: BRK               ; 00 
f9b6: BRK               ; 00 
f9b7: .byte $03   ;?
f9b8: LDX  #$05         ; a2 05
f9ba: RTS               ; 60 
f9bb: BRK               ; 00 
f9bc: .byte $3b   ;?
f9bd: .byte $0f   ;?
f9be: TRB  $12          ; 14 12
f9c0: BRK               ; 00 
f9c1: LSR               ; 4a 
f9c2: LSR               ; 4a 
f9c3: BRK               ; 00 
f9c4: CPY  #$02         ; c0 02
f9c6: .byte $0b   ;?
f9c7: BRK               ; 00 
f9c8: ORA  ($2c,X)      ; 01 2c
f9ca: ASL               ; 0a 
f9cb: AND  $0009,X      ; 3d 09 00
f9ce: PHP               ; 08 
f9cf: SBC  #$00         ; e9 00
f9d1: CMP  ($00,X)      ; c1 00
f9d3: TRB  $0a          ; 14 0a
f9d5: INX               ; e8 
f9d6: .byte $f7   ;?
f9d7: BIT  ($ee12)      ; 3c 12 ee
f9da: ORA  ($0c,X)      ; 01 0c
f9dc: RTI               ; 40 
f9dd: BPL  $fa1f        ; 10 40
f9df: CPX  #$05         ; e0 05
f9e1: DEA               ; 3a 
f9e2: BRK               ; 00 
f9e3: BRK               ; 00 
f9e4: .byte $f9   ;?
f9e5: JSR  $0001        ; 20 01 00
f9e8: RTI               ; 40 
f9e9: BRK               ; 00 
f9ea: BRK               ; 00 
f9eb: BRK               ; 00 
f9ec: BRK               ; 00 
f9ed: BRK               ; 00 
f9ee: BRK               ; 00 
f9ef: BRK               ; 00 
f9f0: BRK               ; 00 
f9f1: .byte $03   ;?
f9f2: BRK               ; 00 
f9f3: RTI               ; 40 
f9f4: BRK               ; 00 
f9f5: BRK               ; 00 
f9f6: BRK               ; 00 
f9f7: BRK               ; 00 
f9f8: BRK               ; 00 
f9f9: BRK               ; 00 
f9fa: BRK               ; 00 
f9fb: BRK               ; 00 
f9fc: .byte $03   ;?
f9fd: BRK               ; 00 
f9fe: RTI               ; 40 
f9ff: BRK               ; 00 
fa00: BRK               ; 00 
fa01: BRK               ; 00 
fa02: BRK               ; 00 
fa03: BRK               ; 00 
fa04: BRK               ; 00 
fa05: BRK               ; 00 
fa06: BRK               ; 00 
fa07: .byte $03   ;?
fa08: BRK               ; 00 
fa09: RTI               ; 40 
fa0a: BRK               ; 00 
fa0b: BRK               ; 00 
fa0c: BRK               ; 00 
fa0d: BRK               ; 00 
fa0e: BRK               ; 00 
fa0f: BRK               ; 00 
fa10: BRK               ; 00 
fa11: BRK               ; 00 
fa12: .byte $03   ;?
fa13: BRK               ; 00 
fa14: RTI               ; 40 
fa15: BRK               ; 00 
fa16: BRK               ; 00 
fa17: BRK               ; 00 
fa18: BRK               ; 00 
fa19: BRK               ; 00 
fa1a: BRK               ; 00 
fa1b: BRK               ; 00 
fa1c: BRK               ; 00 
fa1d: .byte $03   ;?
fa1e: BRK               ; 00 
fa1f: RTI               ; 40 
fa20: BRK               ; 00 
fa21: BRK               ; 00 
fa22: BRK               ; 00 
fa23: BRK               ; 00 
fa24: BRK               ; 00 
fa25: BRK               ; 00 
fa26: BRK               ; 00 
fa27: BRK               ; 00 
fa28: .byte $03   ;?
fa29: BRK               ; 00 
fa2a: RTI               ; 40 
fa2b: BRK               ; 00 
fa2c: BRK               ; 00 
fa2d: BRK               ; 00 
fa2e: BRK               ; 00 
fa2f: BRK               ; 00 
fa30: BRK               ; 00 
fa31: BRK               ; 00 
fa32: BRK               ; 00 
fa33: .byte $03   ;?
fa34: BRK               ; 00 
fa35: RTI               ; 40 
fa36: BRK               ; 00 
fa37: BRK               ; 00 
fa38: BRK               ; 00 
fa39: BRK               ; 00 
fa3a: BRK               ; 00 
fa3b: BRK               ; 00 
fa3c: BRK               ; 00 
fa3d: BRK               ; 00 
fa3e: .byte $03   ;?
fa3f: BRK               ; 00 
fa40: RTI               ; 40 
fa41: BRK               ; 00 
fa42: BRK               ; 00 
fa43: BRK               ; 00 
fa44: BRK               ; 00 
fa45: BRK               ; 00 
fa46: BRK               ; 00 
fa47: BRK               ; 00 
fa48: BRK               ; 00 
fa49: .byte $03   ;?
fa4a: BRK               ; 00 
fa4b: RTI               ; 40 
fa4c: BRK               ; 00 
fa4d: BRK               ; 00 
fa4e: BRK               ; 00 
fa4f: BRK               ; 00 
fa50: BRK               ; 00 
fa51: BRK               ; 00 
fa52: BRK               ; 00 
fa53: BRK               ; 00 
fa54: .byte $03   ;?
fa55: .byte $a7   ;?
fa56: CMP  $d52c,X      ; cd 2c d5
fa59: .byte $a7   ;?
fa5a: CLV               ; b8 
fa5b: .byte $c2   ;?
fa5c: JMP  ($1825,X)    ; 7c 25 18
fa5f: .byte $1b   ;?
fa60: STX  $bd,Y        ; 96 bd
fa62: .byte $23   ;?
fa63: STA  ($d9)        ; 92 d9
fa65: INY               ; c8 
fa66: .byte $57   ;?
fa67: .byte $6b   ;?
fa68: LSR  $0747        ; 5e 47 07
fa6b: .byte $5b   ;?
fa6c: .byte $57   ;?
fa6d: .byte $02   ;?
fa6e: AND  ($46),Y      ; 31 46
fa70: .byte $03   ;?
fa71: SBC  $4894,X      ; ed 94 48
fa74: CPX  $cc          ; e4 cc
fa76: BRK               ; 00 
fa77: SED               ; f8 
fa78: BMI  $fa92        ; 30 18
fa7a: INC  $86e9        ; ee e9 86
fa7d: .byte $22   ;?
fa7e: EOR  ($72),Y      ; 51 72
fa80: .byte $e3   ;?
fa81: .byte $77   ;?
fa82: LDY  ($d6b1)      ; ac b1 d6
fa85: STA  $66          ; 85 66
fa87: .byte $cb   ;?
fa88: BEQ  $fb06        ; f0 7c
fa8a: .byte $5b   ;?
fa8b: ROR  $7f,X        ; 76 7f
fa8d: .byte $77   ;?
fa8e: LDY  $ba,X        ; b4 ba
fa90: ADC  $f8dc,X      ; 7d dc f8
fa93: SBC  $c3          ; f5 c3
fa95: .byte $02   ;?
fa96: STZ  $1e          ; 64 1e
fa98: STY  $13          ; 84 13
fa9a: PLY               ; 7a 
fa9b: .byte $4b   ;?
fa9c: .byte $f7   ;?
fa9d: AND  ($d6),Y      ; 31 d6
fa9f: ROR  $54,X        ; 76 54
faa1: CMP  $7d45,X      ; dd 45 7d
faa4: .byte $02   ;?
faa5: STX  $19,Y        ; 96 19
faa7: ADC  #$10         ; 69 10
faa9: LDX  $f1b8        ; be b8 f1
faac: .byte $e2   ;?
faad: .byte $c2   ;?
faae: STZ  $3f          ; 74 3f
fab0: ROL  $a07b        ; 2e 7b a0
fab3: .byte $f3   ;?
fab4: .byte $27   ;?
fab5: BNE  $fa98        ; d0 e1
fab7: ROR  $f9,X        ; 66 f9
fab9: STZ  $31c3,X      ; 9e c3 31
fabc: ROR  $db,X        ; 76 db
fabe: STA  $10ef,X      ; 8d ef 10
fac1: INC  $56,X        ; f6 56
fac3: .byte $3f   ;?
fac4: .byte $07   ;?
fac5: .byte $82   ;?
fac6: .byte $59   ;?
fac7: ASL  $15e5        ; 0e e5 15
faca: .byte $87   ;?
facb: TRB  $28          ; 14 28
facd: .byte $6b   ;?
face: JMP  ($3e9d,X)    ; 5c 9d 3e
fad1: BNE  $fac7        ; d0 f4
fad3: .byte $8b   ;?
fad4: .byte $b7   ;?
fad5: LDX  $995f        ; be 5f 99
fad8: .byte $b7   ;?
fad9: .byte $c3   ;?
fada: AND  ($e3)        ; 32 e3
fadc: LDY  ($5fa6)      ; ac a6 5f
fadf: .byte $a3   ;?
fae0: .byte $3f   ;?
fae1: DEX               ; ca 
fae2: .byte $47   ;?
fae3: .byte $3f   ;?
fae4: STA  $0ed3,X      ; 8d d3 0e
fae7: STA  ($97,X)      ; 81 97
fae9: BIT  #$30         ; 89 30
faeb: ROL  $f7,X        ; 26 f7
faed: .byte $fb   ;?
faee: .byte $53   ;?
faef: .byte $9b   ;?
faf0: DEC  $e5,X        ; d6 e5
faf2: JMP  ($1c04,X)    ; 5c 04 1c
faf5: TRB  $c5          ; 54 c5
faf7: .byte $7f   ;?
faf8: .byte $e7   ;?
faf9: DEY               ; 88 
fafa: .byte $2f   ;?
fafb: TAY               ; a8 
fafc: .byte $3f   ;?
fafd: .byte $1b   ;?
fafe: BIT  $5d          ; 34 5d
fb00: ORA  #$61         ; 09 61
fb02: BRA  $fac8        ; 80 c4
fb04: DEA               ; 3a 
fb05: BPL  $fb27        ; 10 20
fb07: .byte $8b   ;?
fb08: LSR  $1b22        ; 4e 22 1b
fb0b: BVS  $fb06        ; 70 f9
fb0d: SED               ; f8 
fb0e: RTI               ; 40 
fb0f: .byte $02   ;?
fb10: ORA  $3fc0,X      ; 1d c0 3f
fb13: .byte $a7   ;?
fb14: SBC  $fc1a,X      ; fd 1a fc
fb17: .byte $df   ;?
fb18: BCS  $fb11        ; b0 f7
fb1a: AND  ($c8,X)      ; 21 c8
fb1c: PHX               ; da 
fb1d: .byte $c2   ;?
fb1e: CPX  $e3          ; f4 e3
fb20: .byte $97   ;?
fb21: .byte $c3   ;?
fb22: PLA               ; 68 
fb23: .byte $17   ;?
fb24: CLV               ; b8 
fb25: CMP  ($77,X)      ; c1 77
fb27: .byte $f3   ;?
fb28: .byte $33   ;?
fb29: INX               ; e8 
fb2a: .byte $af   ;?
fb2b: .byte $39   ;?
fb2c: CMP  $eb          ; d5 eb
fb2e: TSB  ($3b99)      ; 0c 99 3b
fb31: .byte $6b   ;?
fb32: .byte $07   ;?
fb33: .byte $bb   ;?
fb34: .byte $02   ;?
fb35: TYA               ; 98 
fb36: SBC  $3b82,X      ; ed 82 3b
fb39: .byte $17   ;?
fb3a: ADC  ($8c)        ; 72 8c
fb3c: .byte $22   ;?
fb3d: BIT  $86          ; 24 86
fb3f: INC  $f910        ; fe 10 f9
fb42: PLA               ; 68 
fb43: .byte $7f   ;?
fb44: .byte $af   ;?
fb45: .byte $6f   ;?
fb46: PLX               ; fa 
fb47: .byte $83   ;?
fb48: ADC  $56e3,X      ; 6d e3 56
fb4b: DEX               ; ca 
fb4c: BIT  $5d          ; 34 5d
fb4e: .byte $9f   ;?
fb4f: AND  #$59         ; 29 59
fb51: SBC  $8a1a,X      ; ed 1a 8a
fb54: .byte $d3   ;?
fb55: LSR  $7d,X        ; 56 7d
fb57: .byte $2f   ;?
fb58: .byte $af   ;?
fb59: .byte $ef   ;?
fb5a: DEY               ; 88 
fb5b: BIT  ($c7c1)      ; 3c c1 c7
fb5e: .byte $d3   ;?
fb5f: ADC  $9c          ; 75 9c
fb61: ORA  #$e8         ; 09 e8
fb63: EOR  $c5          ; 45 c5
fb65: LDY  #$93         ; a0 93
fb67: BRK               ; 00 
fb68: AND  $1c          ; 25 1c
fb6a: ROR  $6b,X        ; 76 6b
fb6c: LDA  $e1d4,X      ; bd d4 e1
fb6f: PLY               ; 7a 
fb70: STZ  $e14e,X      ; 9e 4e e1
fb73: .byte $59   ;?
fb74: .byte $19   ;?
fb75: CPY  ($af97)      ; dc 97 af
fb78: BIT  $89          ; 34 89
fb7a: CPY  ($5e1f)      ; cc 1f 5e
fb7d: AND  $c4          ; 35 c4
fb7f: ORA  ($64),Y      ; 11 64
fb81: CPX  ($b050)      ; fc 50 b0
fb84: LDY  $dd          ; a4 dd
fb86: PLP               ; 28 
fb87: BIT  $81          ; 34 81
fb89: .byte $99   ;?
fb8a: .byte $78   ;?
fb8b: .byte $63   ;?
fb8c: CLC               ; 18 
fb8d: PHX               ; da 
fb8e: .byte $b9   ;?
fb8f: .byte $53   ;?
fb90: AND  ($95)        ; 32 95
fb92: .byte $c3   ;?
fb93: ADC  ($7b)        ; 72 7b
fb95: .byte $b3   ;?
fb96: .byte $22   ;?
fb97: CMP  #$f2         ; c9 f2
fb99: CPX  ($7ea6)      ; ec a6 7e
fb9c: LDA  ($f4,X)      ; a1 f4
fb9e: .byte $3f   ;?
fb9f: .byte $99   ;?
fba0: .byte $8f   ;?
fba1: .byte $db   ;?
fba2: PHP               ; 08 
fba3: ORA  #$f3         ; 09 f3
fba5: LDX  $e7a2        ; ae a2 e7
fba8: .byte $ab   ;?
fba9: .byte $63   ;?
fbaa: CPY  $fa          ; c4 fa
fbac: INC  $11f5        ; ee f5 11
fbaf: JMP  ($cc3e,X)    ; 7c 3e cc
fbb2: ROL               ; 2a 
fbb3: ORA  $61a1,X      ; 1d a1 61
fbb6: ORA  $46          ; 05 46
fbb8: ORA  ($20)        ; 12 20
fbba: ROR  $33a0        ; 7e a0 33
fbbd: STX  $d62c        ; 8e 2c d6
fbc0: EOR  $ac          ; 45 ac
fbc2: .byte $77   ;?
fbc3: .byte $97   ;?
fbc4: .byte $5b   ;?
fbc5: STZ  ($e0e8)      ; 9c e8 e0
fbc8: .byte $cb   ;?
fbc9: .byte $83   ;?
fbca: CMP  ($79,X)      ; c1 79
fbcc: INC  $af,X        ; e6 af
fbce: .byte $cf   ;?
fbcf: .byte $bf   ;?
fbd0: SBC  #$fa         ; e9 fa
fbd2: JMP  ($34a1,X)    ; 7c a1 34
fbd5: .byte $6f   ;?
fbd6: AND  #$ee         ; 29 ee
fbd8: .byte $bb   ;?
fbd9: AND  $cebb,X      ; 2d bb ce
fbdc: .byte $bb   ;?
fbdd: BIT  #$75         ; 89 75
fbdf: .byte $8f   ;?
fbe0: .byte $af   ;?
fbe1: .byte $78   ;?
fbe2: ASL  $a4,X        ; 06 a4
fbe4: .byte $82   ;?
fbe5: AND  $e8e4,X      ; 2d e4 e8
fbe8: .byte $07   ;?
fbe9: .byte $ff   ;?
fbea: .byte $d9   ;?
fbeb: PLP               ; 28 
fbec: TAY               ; a8 
fbed: .byte $8f   ;?
fbee: CPY  ($5be1)      ; dc e1 5b
fbf1: .byte $27   ;?
fbf2: .byte $23   ;?
fbf3: .byte $47   ;?
fbf4: .byte $db   ;?
fbf5: ROR  $76,X        ; 66 76
fbf7: DEY               ; 88 
fbf8: ORA  $1b          ; 15 1b
fbfa: .byte $c7   ;?
fbfb: BVC  $fbde        ; 50 e1
fbfd: .byte $6f   ;?
fbfe: BRK               ; 00 
fbff: STA  $48f4,X      ; 8d f4 48
fc02: ASL  $1375        ; 1e 75 13
fc05: SBC               ; eb 
fc06: JMP  ($89c6,X)    ; 7c c6 89
fc09: SBC  ($72,X)      ; e1 72
fc0b: LSR  $1cdc        ; 4e dc 1c
fc0e: JSR  $bdad        ; 20 ad bd
fc11: .byte $5b   ;?
fc12: STA  $57          ; 85 57
fc14: ROL  $d4cc        ; 3e cc d4
fc17: CMP  $f249,X      ; dd 49 f2
fc1a: PHP               ; 08 
fc1b: ADC  $c74e,X      ; 6d 4e c7
fc1e: .byte $b7   ;?
fc1f: TYA               ; 98 
fc20: BIT  ($2423)      ; 3c 23 24
fc23: TRB  $b3          ; 54 b3
fc25: JMP  ($439a,X)    ; 5c 9a 43
fc28: .byte $79   ;?
fc29: .byte $2f   ;?
fc2a: SBC  $34          ; f5 34
fc2c: .byte $2b   ;?
fc2d: CPX  #$82         ; e0 82
fc2f: INX               ; e8 
fc30: ROL  $9f,X        ; 36 9f
fc32: ASL               ; 0a 
fc33: CLC               ; 18 
fc34: ASL  $2b,X        ; 06 2b
fc36: ROL               ; 2a 
fc37: LDY  $7e,X        ; b4 7e
fc39: TSX               ; ba 
fc3a: .byte $e7   ;?
fc3b: ROR  $19a9        ; 6e a9 19
fc3e: BRA  $fcb7        ; 80 77
fc40: .byte $af   ;?
fc41: BPL  $fbc6        ; 10 83
fc43: BIT  #$0c         ; 89 0c
fc45: ASL               ; 0a 
fc46: BIT  $27          ; 24 27
fc48: SBC  $caf3,X      ; fd f3 ca
fc4b: .byte $22   ;?
fc4c: BIT  ($ec62)      ; 3c 62 ec
fc4f: EOR  $f9ee,X      ; 5d ee f9
fc52: TRB  ($dad2)      ; 1c d2 da
fc55: ASL               ; 0a 
fc56: .byte $62   ;?
fc57: DEC  $dc,X        ; d6 dc
fc59: .byte $7b   ;?
fc5a: CPY  ($9a9a)      ; cc 9a 9a
fc5d: SBC  $f343,X      ; ed 43 f3
fc60: ROR  $19,X        ; 76 19
fc62: LDX  #$5c         ; a2 5c
fc64: CMP  $7c          ; c5 7c
fc66: .byte $02   ;?
fc67: .byte $39   ;?
fc68: .byte $3f   ;?
fc69: BIT  $59          ; 34 59
fc6b: .byte $5f   ;?
fc6c: .byte $b7   ;?
fc6d: ROL               ; 2a 
fc6e: TSB  $d2          ; 44 d2
fc70: ORA  $2172,X      ; 0d 72 21
fc73: DEC  $f062        ; ce 62 f0
fc76: .byte $62   ;?
fc77: ASL               ; 0a 
fc78: STY  ($1e3e)      ; 8c 3e 1e
fc7b: .byte $39   ;?
fc7c: CMP  #$cb         ; c9 cb
fc7e: NOP               ; ea 
fc7f: .byte $d3   ;?
fc80: CLV               ; b8 
fc81: EOR  $e8          ; 45 e8
fc83: .byte $13   ;?
fc84: STX  $39,X        ; 86 39
fc86: ASL  $6642        ; 1e 42 66
fc89: PLX               ; fa 
fc8a: ORA  #$33         ; 09 33
fc8c: EOR  $c0          ; 45 c0
fc8e: .byte $73   ;?
fc8f: .byte $0f   ;?
fc90: LDY  ($8312)      ; bc 12 83
fc93: TRB  $47          ; 54 47
fc95: .byte $67   ;?
fc96: .byte $13   ;?
fc97: .byte $19   ;?
fc98: LDY  $09,X        ; b4 09
fc9a: .byte $47   ;?
fc9b: STA  $df          ; 95 df
fc9d: .byte $3b   ;?
fc9e: JMP  ($9ee0,X)    ; 5c e0 9e
fca1: STZ  ($f9be)      ; 9c be f9
fca4: SED               ; f8 
fca5: .byte $cb   ;?
fca6: LSR  $9b09        ; 5e 09 9b
fca9: EOR  $e744,X      ; 4d 44 e7
fcac: STA  $61ac,X      ; 8d ac 61
fcaf: CMP  $e222,X      ; dd 22 e2
fcb2: TAY               ; a8 
fcb3: .byte $ab   ;?
fcb4: .byte $2f   ;?
fcb5: INC  $5d,X        ; f6 5d
fcb7: .byte $cb   ;?
fcb8: .byte $7f   ;?
fcb9: SBC  $ad          ; e5 ad
fcbb: CPX  $84          ; f4 84
fcbd: CMP  $80          ; c5 80
fcbf: SBC  ($4f,X)      ; e1 4f
fcc1: ROL  $8c,X        ; 26 8c
fcc3: TRB  $c2          ; 54 c2
fcc5: .byte $b9   ;?
fcc6: LDY  ($0557)      ; ac 57 05
fcc9: AND  $70          ; 35 70
fccb: .byte $c3   ;?
fccc: .byte $37   ;?
fccd: ROR  $f6,X        ; 76 f6
fccf: TSB  $68          ; 04 68
fcd1: INX               ; e8 
fcd2: SED               ; f8 
fcd3: TRB  ($25c5)      ; 1c c5 25
fcd6: PLP               ; 28 
fcd7: .byte $e7   ;?
fcd8: ORA  ($db)        ; 12 db
fcda: .byte $b9   ;?
fcdb: STZ  $f15e,X      ; 9e 5e f1
fcde: .byte $3f   ;?
fcdf: .byte $9f   ;?
fce0: .byte $f3   ;?
fce1: SBC               ; eb 
fce2: TSX               ; ba 
fce3: JMP  ($8b8c)      ; 6c 8c 8b
fce6: INC  $ccf3        ; ee f3 cc
fce9: TSX               ; ba 
fcea: INC  $0e,X        ; f6 0e
fcec: CPX  ($19a7)      ; ec a7 19
fcef: .byte $4b   ;?
fcf0: LDA  $2e          ; b5 2e
fcf2: CMP  $aeee,X      ; dd ee ae
fcf5: JMP  ($355b,X)    ; 7c 5b 35
fcf8: .byte $97   ;?
fcf9: SBC  ($d3)        ; f2 d3
fcfb: CPY  ($4717)      ; dc 17 47
fcfe: STA  ($e2,X)      ; 81 e2
fd00: LDA  ($f3)        ; b2 f3
fd02: .byte $af   ;?
fd03: .byte $8b   ;?
fd04: .byte $ff   ;?
fd05: CMP  #$0c         ; c9 0c
fd07: LDX  $18,X        ; a6 18
fd09: CPX  $3e          ; e4 3e
fd0b: ORA  ($53),Y      ; 11 53
fd0d: CMP  $b3          ; c5 b3
fd0f: .byte $7b   ;?
fd10: AND  ($ba),Y      ; 31 ba
fd12: LDX  $946a        ; be 6a 94
fd15: .byte $83   ;?
fd16: .byte $df   ;?
fd17: ROR  $3135        ; 7e 35 31
fd1a: .byte $63   ;?
fd1b: INC  $56,X        ; f6 56
fd1d: BVC  $fd4d        ; 50 2e
fd1f: ADC  ($b5),Y      ; 71 b5
fd21: LSR  $d5,X        ; 46 d5
fd23: INC  $c2,X        ; f6 c2
fd25: .byte $fb   ;?
fd26: BEQ  $fd5f        ; f0 37
fd28: ORA  ($bb)        ; 12 bb
fd2a: .byte $af   ;?
fd2b: .byte $13   ;?
fd2c: TRB  $59          ; 14 59
fd2e: LDA  $045c,X      ; ad 5c 04
fd31: STX  $02,Y        ; 96 02
fd33: CLD               ; d8 
fd34: ASL  $9cc2        ; 1e c2 9c
fd37: TXA               ; 8a 
fd38: ORA  $a2          ; 15 a2
fd3a: BNE  $fd6f        ; d0 33
fd3c: .byte $c3   ;?
fd3d: STA  ($2d)        ; 92 2d
fd3f: TRB  $23          ; 14 23
fd41: CPX  ($f892)      ; ec 92 f8
fd44: ADC  $81a9,X      ; 7d a9 81
fd47: .byte $6f   ;?
fd48: SBC  ($47,X)      ; e1 47
fd4a: .byte $b3   ;?
fd4b: EOR  $ecd8,X      ; 5d d8 ec
fd4e: CPX  ($57b4)      ; ec b4 57
fd51: CMP  #$12         ; c9 12
fd53: .byte $7f   ;?
fd54: CPX  $c1          ; f4 c1
fd56: PLY               ; 7a 
fd57: JSR  $4c5d        ; 20 5d 4c
fd5a: STY  $dc          ; 84 dc
fd5c: .byte $07   ;?
fd5d: .byte $97   ;?
fd5e: .byte $67   ;?
fd5f: STA  $24          ; 95 24
fd61: SBC  $d64f,X      ; ed 4f d6
fd64: .byte $d9   ;?
fd65: .byte $5b   ;?
fd66: LDX  $16,X        ; a6 16
fd68: LDX  $b090        ; ae 90 b0
fd6b: .byte $37   ;?
fd6c: LSR  $04,X        ; 46 04
fd6e: .byte $5b   ;?
fd6f: LSR  $5b51        ; 4e 51 5b
fd72: LSR  $a7,X        ; 56 a7
fd74: .byte $bf   ;?
fd75: ORA  #$61         ; 09 61
fd77: ORA  #$38         ; 09 38
fd79: ROL  $e5,X        ; 36 e5
fd7b: SEC               ; 38 
fd7c: CMP  #$e5         ; c9 e5
fd7e: CMP  $b9          ; c5 b9
fd80: SBC  ($52,X)      ; e1 52
fd82: STZ  $db          ; 74 db
fd84: ADC  $489a,X      ; 7d 9a 48
fd87: .byte $f3   ;?
fd88: LDY  #$9f         ; a0 9f
fd8a: .byte $0f   ;?
fd8b: INX               ; e8 
fd8c: ASL  $2b,X        ; 06 2b
fd8e: SBC  ($f0,X)      ; e1 f0
fd90: INA               ; 1a 
fd91: .byte $53   ;?
fd92: ROL  $9a,X        ; 36 9a
fd94: ROL  $20,X        ; 26 20
fd96: SEC               ; 38 
fd97: ORA  $6acb,X      ; 0d cb 6a
fd9a: JMP  ($b290,X)    ; 5c 90 b2
fd9d: ASL  $2d,X        ; 06 2d
fd9f: .byte $4b   ;?
fda0: STA  ($21,X)      ; 81 21
fda2: PLA               ; 68 
fda3: EOR  $ec11,X      ; 4d 11 ec
fda6: .byte $4f   ;?
fda7: .byte $cf   ;?
fda8: .byte $df   ;?
fda9: PLY               ; 7a 
fdaa: SEC               ; 38 
fdab: CLV               ; b8 
fdac: CMP  ($e7,X)      ; c1 e7
fdae: ADC  $63          ; 65 63
fdb0: .byte $db   ;?
fdb1: .byte $bb   ;?
fdb2: .byte $19   ;?
fdb3: STA  $8fd2,X      ; 9d d2 8f
fdb6: SBC  #$f3         ; e9 f3
fdb8: RTI               ; 40 
fdb9: .byte $ef   ;?
fdba: BVC  $fde7        ; 50 2b
fdbc: PLA               ; 68 
fdbd: BRA  $fd46        ; 80 87
fdbf: ORA  ($2c),Y      ; 11 2c
fdc1: .byte $63   ;?
fdc2: STA  $74          ; 95 74
fdc4: AND  $a3da,X      ; 3d da a3
fdc7: ADC  ($93),Y      ; 71 93
fdc9: ROL               ; 2a 
fdca: STA  ($f0)        ; 92 f0
fdcc: ORA  $d0c2,X      ; 1d c2 d0
fdcf: ROR  $9abb        ; 6e bb 9a
fdd2: PLA               ; 68 
fdd3: .byte $fb   ;?
fdd4: ADC  #$df         ; 69 df
fdd6: PHY               ; 5a 
fdd7: .byte $83   ;?
fdd8: LDX  $2bcd        ; be cd 2b
fddb: EOR  ($95)        ; 52 95
fddd: CMP  $d3bd,X      ; dd bd d3
fde0: .byte $0b   ;?
fde1: AND  ($61,X)      ; 21 61
fde3: .byte $99   ;?
fde4: TAX               ; aa 
fde5: SBC  $a22b,X      ; ed 2b a2
fde8: SBC  $b8          ; e5 b8
fdea: .byte $87   ;?
fdeb: CLD               ; d8 
fdec: CMP  $42          ; c5 42
fdee: ASL               ; 0a 
fdef: .byte $d9   ;?
fdf0: .byte $e2   ;?
fdf1: SED               ; f8 
fdf2: STY  ($95fa)      ; 8c fa 95
fdf5: STX  $74,Y        ; 96 74
fdf7: STX  $6d,X        ; 86 6d
fdf9: TSB  ($4c53)      ; 0c 53 4c
fdfc: SBC  ($a4,X)      ; e1 a4
fdfe: LDA  $5a          ; b5 5a
fe00: EOR  $62          ; 45 62
fe02: AND  $04          ; 35 04
fe04: .byte $ff   ;?
fe05: .byte $ab   ;?
fe06: CMP  ($29),Y      ; d1 29
fe08: CPX  ($7d2b)      ; fc 2b 7d
fe0b: CLV               ; b8 
fe0c: LSR  $fbf1        ; 5e f1 fb
fe0f: ROR  $dd78        ; 6e 78 dd
fe12: SBC               ; eb 
fe13: .byte $e2   ;?
fe14: SBC  $1f3e,X      ; fd 3e 1f
fe17: CPY  $76          ; c4 76
fe19: LDY  $31,X        ; b4 31
fe1b: .byte $62   ;?
fe1c: SBC  ($5c)        ; f2 5c
fe1e: .byte $59   ;?
fe1f: ROR  $2d02        ; 6e 02 2d
fe22: .byte $79   ;?
fe23: CPX  ($9ee5)      ; fc e5 9e
fe26: .byte $7b   ;?
fe27: .byte $cf   ;?
fe28: BRK               ; 00 
fe29: TXA               ; 8a 
fe2a: CMP  $92d6,X      ; dd d6 92
fe2d: LSR  $8b,X        ; 56 8b
fe2f: .byte $f3   ;?
fe30: LDX  $52,X        ; a6 52
fe32: SBC  $4afc,X      ; ed fc 4a
fe35: EOR  $0442,X      ; 5d 42 04
fe38: BRK               ; 00 
fe39: CMP  $dc2d,X      ; cd 2d dc
fe3c: ADC  ($8f)        ; 72 8f
fe3e: .byte $8f   ;?
fe3f: DEC  $6a,X        ; d6 6a
fe41: STY  ($ca95)      ; 8c 95 ca
fe44: .byte $03   ;?
fe45: EOR  $5c12,X      ; 4d 12 5c
fe48: INA               ; 1a 
fe49: .byte $df   ;?
fe4a: .byte $0f   ;?
fe4b: ADC  $f908,X      ; 7d 08 f9
fe4e: LDA  ($30,X)      ; a1 30
fe50: .byte $93   ;?
fe51: EOR  $8ffc,X      ; 5d fc 8f
fe54: .byte $23   ;?
fe55: ADC  $0fc7,X      ; 7d c7 0f
fe58: .byte $33   ;?
fe59: RTS               ; 60 
fe5a: .byte $af   ;?
fe5b: ORA  ($08,X)      ; 01 08
fe5d: CPY  $3f          ; c4 3f
fe5f: PHY               ; 5a 
fe60: LDX  $86,X        ; a6 86
fe62: BEQ  $fe19        ; f0 b5
fe64: ORA  ($3f,X)      ; 01 3f
fe66: ROR  $7097        ; 7e 97 70
fe69: AND  ($7b),Y      ; 31 7b
fe6b: .byte $f3   ;?
fe6c: PLA               ; 68 
fe6d: .byte $62   ;?
fe6e: ROL  $ee,X        ; 26 ee
fe70: PLA               ; 68 
fe71: .byte $07   ;?
fe72: ROR  $30e5        ; 7e e5 30
fe75: AND  $94          ; 25 94
fe77: LDA  $7337,X      ; ad 37 73
fe7a: .byte $1f   ;?
fe7b: INA               ; 1a 
fe7c: EOR  $5383,X      ; 5d 83 53
fe7f: CPY  $dd          ; c4 dd
fe81: .byte $22   ;?
fe82: CPX  $06          ; f4 06
fe84: .byte $87   ;?
fe85: INC  $49a6        ; fe a6 49
fe88: AND  $8888,X      ; 2d 88 88
fe8b: BRA  $fee8        ; 80 5b
fe8d: .byte $19   ;?
fe8e: .byte $02   ;?
fe8f: SBC  $4c          ; f5 4c
fe91: CLI               ; 58 
fe92: CMP  $1a43,X      ; dd 43 1a
fe95: CPY  ($a492)      ; dc 92 a4
fe98: AND  $0f          ; 35 0f
fe9a: CPX  $6e          ; e4 6e
fe9c: CLC               ; 18 
fe9d: TRB  $b2          ; 54 b2
fe9f: PHX               ; da 
fea0: SEC               ; 38 
fea1: ROR               ; 6a 
fea2: .byte $07   ;?
fea3: ORA  $3e          ; 05 3e
fea5: ORA  ($74),Y      ; 11 74
fea7: .byte $39   ;?
fea8: ROR  $eb,X        ; 76 eb
feaa: LDY  #$6d         ; a0 6d
feac: DEY               ; 88 
fead: LDX  $1d,X        ; a6 1d
feaf: TSB  ($5967)      ; 0c 67 59
feb2: .byte $7f   ;?
feb3: CPX  $88          ; f4 88
feb5: BIT  ($bfa9)      ; 2c a9 bf
feb8: TRB  $3d          ; 54 3d
feba: .byte $67   ;?
febb: LSR  $e1,X        ; 46 e1
febd: .byte $bf   ;?
febe: .byte $43   ;?
febf: STZ  $de          ; 64 de
fec1: ADC  $92          ; 65 92
fec3: .byte $f9   ;?
fec4: CMP  $ce00,X      ; cd 00 ce
fec7: BMI  $ff3e        ; 30 75
fec9: STZ  $29          ; 64 29
fecb: .byte $02   ;?
fecc: .byte $97   ;?
fecd: STA  ($5f),Y      ; 91 5f
fecf: .byte $fb   ;?
fed0: SBC  ($10,X)      ; e1 10
fed2: CMP  $223d,X      ; cd 3d 22
fed5: STZ  ($c4c0)      ; 9c c0 c4
fed8: LDA  $1103,X      ; bd 03 11
fedb: NOP               ; ea 
fedc: RTS               ; 60 
fedd: EOR  $2bef,X      ; 4d ef 2b
fee0: .byte $f7   ;?
fee1: .byte $1b   ;?
fee2: CPX  ($dc2a)      ; fc 2a dc
fee5: .byte $53   ;?
fee6: STA  $51f5,X      ; 9d f5 51
fee9: PLX               ; fa 
feea: STY  ($3186)      ; 8c 86 31
feed: .byte $62   ;?
feee: BCS  $fe7c        ; b0 8c
fef0: .byte $e3   ;?
fef1: .byte $87   ;?
fef2: ROL  $9069        ; 2e 69 90
fef5: LDY  $c1,X        ; b4 c1
fef7: JMP  ($6b97,X)    ; 7c 97 6b
fefa: .byte $47   ;?
fefb: LDA  ($3e,X)      ; a1 3e
fefd: STY  $9e          ; 84 9e
feff: INC  $ab,X        ; f6 ab
ff01: STY  $af,X        ; 94 af
ff03: SBC  ($14,X)      ; e1 14
ff05: STA  ($c9)        ; 92 c9
ff07: EOR  ($33),Y      ; 51 33
ff09: LDA  $30          ; b5 30
ff0b: .byte $42   ;?
ff0c: SBC  $f7a2,X      ; fd a2 f7
ff0f: STY  $94          ; 84 94
ff11: CPY  $0c          ; c4 0c
ff13: LDX  $fc7b        ; be 7b fc
ff16: STA  ($ae)        ; 92 ae
ff18: ASL               ; 0a 
ff19: .byte $7b   ;?
ff1a: STZ  $b3          ; 74 b3
ff1c: .byte $22   ;?
ff1d: SBC  $8e38,X      ; ed 38 8e
ff20: .byte $23   ;?
ff21: CLD               ; d8 
ff22: ORA  ($66),Y      ; 11 66
ff24: TRB  ($3945)      ; 1c 45 39
ff27: .byte $0b   ;?
ff28: .byte $42   ;?
ff29: STZ  $7d7f,X      ; 9e 7f 7d
ff2c: .byte $a3   ;?
ff2d: .byte $d7   ;?
ff2e: ORA  $1dae,X      ; 1d ae 1d
ff31: CMP  ($cd),Y      ; d1 cd
ff33: STY  $8a          ; 84 8a
ff35: CPX  ($231c)      ; fc 1c 23
ff38: ROL  $ff3d        ; 3e 3d ff
ff3b: ADC  #$b5         ; 69 b5
ff3d: BPL  $fee3        ; 10 a4
ff3f: .byte $bb   ;?
ff40: STX  $f0,X        ; 86 f0
ff42: STY  ($9c00)      ; 8c 00 9c
ff45: CPX  $d7          ; f4 d7
ff47: CPX  ($bba6)      ; ec a6 bb
ff4a: .byte $df   ;?
ff4b: BRA  $ff19        ; 80 cc
ff4d: TXA               ; 8a 
ff4e: ASL               ; 0a 
ff4f: BNE  $fef5        ; d0 a4
ff51: LDA  $da          ; b5 da
ff53: TSB  $66          ; 04 66
ff55: BIT  ($a247)      ; 3c 47 a2
ff58: JMP  ($4fb8,X)    ; 5c b8 4f
ff5b: .byte $0b   ;?
ff5c: STZ  $cb76,X      ; 9e 76 cb
ff5f: TSB  $fc          ; 44 fc
ff61: .byte $e2   ;?
ff62: STX  $f8,X        ; 86 f8
ff64: BIT  ($8927)      ; 3c 27 89
ff67: LDY  ($1587)      ; bc 87 15
ff6a: TSX               ; ba 
ff6b: .byte $93   ;?
ff6c: .byte $1f   ;?
ff6d: TSB  $a6          ; 44 a6
ff6f: LDA  ($53)        ; b2 53
ff71: .byte $78   ;?
ff72: .byte $9b   ;?
ff73: STX  $b5,X        ; 86 b5
ff75: EOR  $fb          ; 45 fb
ff77: CPX  $8b          ; f4 8b
ff79: CPX  $cd          ; f4 cd
ff7b: CPX  #$85         ; e0 85
ff7d: LDA  $96          ; a5 96
ff7f: SBC  ($ae)        ; f2 ae
ff81: .byte $b7   ;?
ff82: .byte $67   ;?
ff83: STX  $a5,Y        ; 96 a5
ff85: ROR  $38,X        ; 66 38
ff87: .byte $42   ;?
ff88: ASL  $76b4        ; 0e b4 76
ff8b: SBC  $8b44,X      ; fd 44 8b
ff8e: CLD               ; d8 
ff8f: ORA  $2152,X      ; 1d 52 21
ff92: .byte $78   ;?
ff93: EOR  ($21),Y      ; 51 21
ff95: STA  $e01f,X      ; 9d 1f e0
ff98: CPX  $f8          ; f4 f8
ff9a: SBC  ($31,X)      ; e1 31
ff9c: ORA  ($27)        ; 12 27
ff9e: SBC               ; eb 
ff9f: STY  ($78c3)      ; 8c c3 78
ffa2: .byte $99   ;?
ffa3: AND  ($7e,X)      ; 21 7e
ffa5: DEA               ; 3a 
ffa6: CPY  ($6247)      ; cc 47 62
ffa9: LDA  ($7c,X)      ; a1 7c
ffab: ASL  $e4,X        ; 06 e4
ffad: SBC  #$8a         ; e9 8a
ffaf: .byte $d9   ;?
ffb0: .byte $c7   ;?
ffb1: TSX               ; ba 
ffb2: CLD               ; d8 
ffb3: LDX  $cd52        ; ae 52 cd
ffb6: STZ  ($bd2f)      ; 9c 2f bd
ffb9: DEC  $46f3        ; ce f3 46
ffbc: INC  $b283        ; ee 83 b2
ffbf: .byte $db   ;?
ffc0: .byte $63   ;?
ffc1: .byte $2b   ;?
ffc2: .byte $82   ;?
ffc3: SBC  ($70,X)      ; e1 70
ffc5: AND  #$2d         ; 29 2d
ffc7: STA  ($ab)        ; 92 ab
ffc9: .byte $e2   ;?
ffca: .byte $33   ;?
ffcb: SED               ; f8 
ffcc: LDA  $e5          ; a5 e5
ffce: LDX  #$ad         ; a2 ad
ffd0: CPY  ($254a)      ; dc 4a 25
ffd3: CPX  $ee          ; e4 ee
ffd5: LSR  $a95b        ; 5e 5b a9
ffd8: BPL  $10057       ; 10 7d
ffda: ROR               ; 6a 
ffdb: ADC  $03          ; 65 03
ffdd: CMP  ($6b,X)      ; c1 6b
ffdf: .byte $1b   ;?
ffe0: LSR  $4ea5        ; 4e a5 4e
ffe3: LDA  $4e          ; a5 4e
ffe5: LDA  $4e          ; a5 4e
ffe7: LDA  $4e          ; a5 4e
ffe9: LDA  $4e          ; a5 4e
ffeb: LDA  $41          ; a5 41
ffed: LDA  ($4e)        ; b2 4e
ffef: LDA  $6a          ; a5 6a
fff1: LDA  ($4f,X)      ; a1 4f
fff3: LDA  $2d          ; a5 2d
fff5: LDA  ($a8,X)      ; a1 a8
fff7: LDY  #$59         ; a0 59
fff9: LDA  $a3          ; a5 a3
fffb: LDY  #$00         ; a0 00
fffd: LDY  #$03         ; a0 03

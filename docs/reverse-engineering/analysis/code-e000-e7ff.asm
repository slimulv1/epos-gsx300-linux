e001: LDA $1016
e004: AND #$0f
e006: STA $40
e008: LDA $0f2a
e00b: AND #$0f
e00d: CMP $40
e00f: BNE $17
e011: LDA $0f2b
e014: AND #$0f
e016: CMP $40
e018: BNE $0e
e01a: LDA $0f20
e01d: AND #$3c
e01f: LSR A
e020: LSR A
e021: CMP #$00
e023: CMP $40
e025: BNE $01
e027: RTS
e028: LDA $40
e02a: STA $4b
e02c: LDA $0f2a
e02f: AND #$f0
e031: ORA $4b
e033: STA $0f2a
e036: LDA $40
e038: STA $4b
e03a: LDA $0f2b
e03d: AND #$f0
e03f: ORA $4b
e041: STA $0f2b
e044: LDA $40
e046: ASL A
e047: ASL A
e048: STA $4b
e04a: LDA $0f20
e04d: AND #$c3
e04f: ORA $4b
e051: STA $0f20
e054: LDA (,zp)$0a
e056: BBS2 $27 → $e059
e059: RTS
e05a: INC A
e05b: STA $40
e05d: TYA
e05e: INC A
e05f: ASL $40
e061: ROL A
e062: STA $40
e064: LDA (,zp)$ab
e066: 44 !UNDEF
e067: JSR $c9aa
e06a: LDA $41
e06c: LSR A
e06d: RTS
e06e: BMI $08
e070: CMP #$06
e072: BCC $0a
e074: LDA #$05
e076: BRA $06
e078: CMP #$b6
e07a: BCS $02
e07c: LDA #$b6
e07e: SEC
e07f: SBC #$05
e081: RTS
e082: BBR2 $a3 → $e085
e085: STA $40
e087: TYA
e088: JSR $e06e
e08b: STZ $40
e08d: BEQ $06
e08f: CMP #$b1
e091: BCS $02
e093: LDA #$b1
e095: RTS
e096: BBR2 $a3 → $e099
e099: BBS2 $4f → $e09c
e09c: CPY $61
e09e: BNE $02
e0a0: LDY #$80
e0a2: JSR $e082
e0a5: RTS
e0a6: JSR $13ff
e0a9: SBC (,zp)$13
e0ab: BPL $80
e0ad: 03 !UNDEF
e0ae: JMP $e129
e0b1: BBS5 $94 → $e0b4
e0b4: RTS
e0b5: BBR4 $4d → $e0b8
e0b8: BBR0 $9d → $e0bb
e0bb: SBC (,zp)$1a
e0bd: BBR0 $80 → $e0c0
e0c0: RMB2 $9a
e0c2: BRA $02
e0c4: SMB2 $9a
e0c6: c2 !UNDEF
e0c7: PHP
e0c8: INC A
e0c9: BBR0 $2f → $e0cc
e0cc: ORA $c7f2
e0cf: TSB $08
e0d1: PHP
e0d2: LDA $0f44
e0d5: AND #$3f
e0d7: STA $0f44
e0da: JSR $148c
e0dd: RMB0 $9d
e0df: BBS4 $9a → $e0e2
e0e2: JSR $e18d
e0e5: LDA $0f18
e0e8: AND #$03
e0ea: BEQ $0b
e0ec: LDA $0f71
e0ef: AND #$03
e0f1: BEQ $04
e0f3: JSR $e167
e0f6: RTS
e0f7: JSR $e174
e0fa: RTS
e0fb: BBR4 $4f → $e0fe
e0fe: SBC (,zp)$14
e100: BPL $80
e102: RMB0 $f2
e104: TRB $10
e106: ORA (,X)$13
e108: BRA $03
e10a: BBS7 $9a → $e10d
e10d: LDA $0f18
e110: AND #$03
e112: BEQ $0e
e114: LDA $0f71
e117: AND #$03
e119: BEQ $07
e11b: JSR $e17d
e11e: JSR $e167
e121: RTS
e122: JSR $e18d
e125: JSR $e174
e128: RTS
e129: BBR4 $4f → $e12c
e12c: SBC (,zp)$14
e12e: BPL $80
e130: RMB0 $f2
e132: TRB $10
e134: ORA (,X)$13
e136: BRA $03
e138: BBS7 $9a → $e13b
e13b: LDA $0f18
e13e: AND #$03
e140: BEQ $22
e142: LDA $0f71
e145: AND #$03
e147: BEQ $1b
e149: e2 !UNDEF
e14a: 13 !UNDEF
e14b: BPL $01
e14d: ORA $20
e14f: ADC ,X$80e1
e152: 03 !UNDEF
e153: JSR $e18d
e156: e2 !UNDEF
e157: 13 !UNDEF
e158: BPL $02
e15a: ORA $20
e15c: RMB6 $e1
e15e: BRA $03
e160: JSR $e174
e163: RTS
e164: JMP $e174
e167: LDA #$00
e169: e2 !UNDEF
e16a: 13 !UNDEF
e16b: BPL $20
e16d: 02 !UNDEF
e16e: ORA #$88
e170: STA $0f1b
e173: RTS
e174: LDA $0f1b
e177: ORA #$33
e179: STA $0f1b
e17c: RTS
e17d: JSR $145c
e180: CMP (,zp)$04
e182: BBR3 $0f → $e185
e185: TRB $090f
e188: BVC $8d
e18a: TRB $600f
e18d: JSR $145f
e190: LDA $0f1c
e193: AND #$af
e195: STA $0f1c
e198: RTS
e199: JSR $1402
e19c: JSR $e3af
e19f: SBC (,zp)$70
e1a1: BBR0 $01 → $e1a4
e1a4: JSR $e221
e1a7: LDA #$00
e1a9: STA $0f23
e1ac: JMP $e410
e1af: e2 !UNDEF
e1b0: ASL ,X$10
e1b2: BRA $0b
e1b4: LDA #$11
e1b6: STA $0f23
e1b9: JSR $e1ca
e1bc: JMP $e410
e1bf: LDA #$11
e1c1: STA $0f23
e1c4: JSR $e221
e1c7: JMP $e22f
e1ca: LDA #$01
e1cc: ASL A
e1cd: ASL A
e1ce: ASL A
e1cf: ASL A
e1d0: ASL A
e1d1: ASL A
e1d2: STA $4b
e1d4: LDA $0f1d
e1d7: AND #$3f
e1d9: ORA $4b
e1db: STA $0f1d
e1de: SBC (,zp)$15
e1e0: BPL $80
e1e2: ASL A
e1e3: c2 !UNDEF
e1e4: BRA $24
e1e6: BBR0 $c2 → $e1e9
e1e9: AND $0f
e1eb: BRA $08
e1ed: CMP (,zp)$80
e1ef: BIT $0f
e1f1: CMP (,zp)$80
e1f3: AND $0f
e1f5: CMP (,zp)$40
e1f7: BIT $0f
e1f9: CMP (,zp)$40
e1fb: AND $0f
e1fd: LDA $1015
e200: AND #$0f
e202: PHA
e203: ASL A
e204: ASL A
e205: STA $4b
e207: LDA $0f21
e20a: AND #$c3
e20c: ORA $4b
e20e: STA $0f21
e211: PLA
e212: ASL A
e213: ASL A
e214: STA $4b
e216: LDA $0f21
e219: AND #$c3
e21b: ORA $4b
e21d: STA $0f21
e220: RTS
e221: LDA $0f24
e224: AND #$bf
e226: ORA #$80
e228: STA $0f24
e22b: STA $0f25
e22e: RTS
e22f: JSR $1477
e232: e2 !UNDEF
e233: RMB1 $10
e235: BRA $22
e237: LDA #$00
e239: ASL A
e23a: ASL A
e23b: ASL A
e23c: ASL A
e23d: ASL A
e23e: ASL A
e23f: STA $4b
e241: LDA $0f1d
e244: AND #$3f
e246: ORA $4b
e248: STA $0f1d
e24b: CMP (,zp)$01
e24d: EOR (,X)$08
e24f: CMP (,zp)$02
e251: EOR (,X)$08
e253: JSR $e44c
e256: JMP $e41e
e259: JSR $e491
e25c: BBR2 $4d → $e25f
e25f: SBC (,zp)$c7
e261: TSB $08
e263: ORA #$d2
e265: 02 !UNDEF
e266: JSR $c20f
e269: ORA (,X)$20
e26b: BBR0 $60 → $e26e
e26e: JSR $010f
e271: RTI
e272: LDA #$03
e274: SBC (,zp)$c7
e276: TSB $04
e278: 02 !UNDEF
e279: LDA #$02
e27b: ASL A
e27c: ASL A
e27d: ASL A
e27e: ASL A
e27f: ASL A
e280: ASL A
e281: STA $4b
e283: LDA $0f1d
e286: AND #$3f
e288: ORA $4b
e28a: STA $0f1d
e28d: LDA #$00
e28f: ASL A
e290: ASL A
e291: STA $4b
e293: LDA $0f20
e296: AND #$c3
e298: ORA $4b
e29a: STA $0f20
e29d: CMP (,zp)$80
e29f: BBR1 $0f → $e2a2
e2a2: BRA $1e
e2a4: BBR0 $c2 → $e2a7
e2a7: JSR $d20f
e2aa: ORA (,X)$20
e2ac: BBR0 $b2 → $e2af
e2af: BBS2 $27 → $e2b2
e2b2: RTS
e2b3: LDA #$00
e2b5: ASL A
e2b6: ASL A
e2b7: ASL A
e2b8: ASL A
e2b9: ASL A
e2ba: ASL A
e2bb: STA $4b
e2bd: LDA $0f1d
e2c0: AND #$3f
e2c2: ORA $4b
e2c4: STA $0f1d
e2c7: SBC (,zp)$2a
e2c9: BBR0 $10 → $e2cc
e2cc: BBR2 $94 → $e2cf
e2cf: LDA #$00
e2d1: STA $4b
e2d3: LDA $0f2a
e2d6: AND #$f0
e2d8: ORA $4b
e2da: STA $0f2a
e2dd: LDA #$00
e2df: STA $4b
e2e1: LDA $0f2b
e2e4: AND #$f0
e2e6: ORA $4b
e2e8: STA $0f2b
e2eb: CMP (,zp)$80
e2ed: BBR1 $0f → $e2f0
e2f0: BRA $1e
e2f2: BBR0 $b2 → $e2f5
e2f5: BBS2 $27 → $e2f8
e2f8: SBC (,zp)$16
e2fa: BPL $40
e2fc: ASL $c2
e2fe: JSR $0f2a
e301: BRA $4b
e303: SBC (,zp)$2a
e305: BBR0 $20 → $e308
e308: CMP (,zp)$80
e30a: ASL ,X$a90f
e30d: ORA (,X)$85
e30f: 4b !UNDEF
e310: LDA $0f2a
e313: AND #$f0
e315: ORA $4b
e317: STA $0f2a
e31a: LDA (,zp)$1e
e31c: BBS2 $27 → $e31f
e31f: e2 !UNDEF
e320: ASL ,X$10
e322: BPL $14
e324: CMP (,zp)$40
e326: ROL A
e327: BBR0 $a9 → $e32a
e32a: STA $4b
e32c: LDA $089e
e32f: AND #$fc
e331: ORA $4b
e333: STA $089e
e336: BRA $12
e338: c2 !UNDEF
e339: RTI
e33a: ROL A
e33b: BBR0 $a9 → $e33e
e33e: STA $4b
e340: LDA $089e
e343: AND #$fc
e345: ORA $4b
e347: STA $089e
e34a: CMP (,zp)$20
e34c: ROL A
e34d: BBR0 $d2 → $e350
e350: ROL A
e351: BBR0 $d2 → $e354
e354: BMI $0f
e356: c2 !UNDEF
e357: ORA (,X)$30
e359: BBR0 $f2 → $e35c
e35c: BBR0 $10 → $e35f
e35f: BBR2 $94 → $e362
e362: LDA (,zp)$0a
e364: BBS2 $27 → $e367
e367: SBC (,zp)$16
e369: BPL $40
e36b: ASL $c2
e36d: JSR $0f2b
e370: BRA $2f
e372: SBC (,zp)$2b
e374: BBR0 $20 → $e377
e377: CMP (,zp)$80
e379: BBR1 $0f → $e37c
e37c: ORA (,X)$85
e37e: 4b !UNDEF
e37f: LDA $0f2b
e382: AND #$f0
e384: ORA $4b
e386: STA $0f2b
e389: LDA (,zp)$1e
e38b: BBS2 $27 → $e38e
e38e: e2 !UNDEF
e38f: ASL ,X$10
e391: BPL $06
e393: CMP (,zp)$40
e395: 2b !UNDEF
e396: BBR0 $80 → $e399
e399: c2 !UNDEF
e39a: RTI
e39b: 2b !UNDEF
e39c: BBR0 $d2 → $e39f
e39f: 2b !UNDEF
e3a0: BBR0 $d2 → $e3a3
e3a3: 2b !UNDEF
e3a4: BBR0 $d2 → $e3a7
e3a7: BMI $0f
e3a9: c2 !UNDEF
e3aa: TSB $30
e3ac: BBR0 $60 → $e3af
e3af: JSR $147d
e3b2: BBR2 $4d → $e3b5
e3b5: BBS0 $9d → $e3b8
e3b8: SBC (,zp)$c7
e3ba: TSB $08
e3bc: 1b !UNDEF
e3bd: c2 !UNDEF
e3be: JSR $0895
e3c1: LDA #$00
e3c3: STA $4b
e3c5: LDA $0894
e3c8: AND #$fc
e3ca: ORA $4b
e3cc: STA $0894
e3cf: LDA $0f44
e3d2: AND #$3f
e3d4: STA $0f44
e3d7: RTS
e3d8: e2 !UNDEF
e3d9: ASL ,X$10
e3db: BPL $06
e3dd: c2 !UNDEF
e3de: JSR $0895
e3e1: BRA $04
e3e3: CMP (,zp)$20
e3e5: STA ,X$08
e3e7: LDA #$02
e3e9: STA $4b
e3eb: LDA $0894
e3ee: AND #$fc
e3f0: ORA $4b
e3f2: STA $0894
e3f5: SBC (,zp)$44
e3f7: BBR0 $40 → $e3fa
e3fa: CMP (,zp)$80
e3fc: BBR1 $0f → $e3ff
e3ff: BRA $1e
e401: BBR0 $b2 → $e404
e404: BBS2 $27 → $e407
e407: LDA $0f44
e40a: ORA #$c0
e40c: STA $0f44
e40f: RTS
e410: JSR $147a
e413: CMP (,zp)$40
e415: RMB3 $0f
e417: CMP (,zp)$40
e419: AND ,Y$200f
e41c: STA (,Y)$e4
e41e: c2 !UNDEF
e41f: 02 !UNDEF
e420: BMI $0f
e422: CMP (,zp)$01
e424: BMI $0f
e426: c2 !UNDEF
e427: PHP
e428: BMI $0f
e42a: CMP (,zp)$04
e42c: BMI $0f
e42e: c2 !UNDEF
e42f: BPL $2a
e431: BBR0 $c2 → $e434
e434: 2b !UNDEF
e435: BBR0 $d2 → $e438
e438: JSR $c20f
e43b: ORA (,X)$20
e43d: BBR0 $f2 → $e440
e440: BPL $20
e442: PHP
e443: c2 !UNDEF
e444: JSR $0f2a
e447: c2 !UNDEF
e448: JSR $0f2b
e44b: RTS
e44c: LDA $1017
e44f: AND #$07
e451: PHA
e452: ASL A
e453: ASL A
e454: ASL A
e455: ASL A
e456: ASL A
e457: STA $4b
e459: LDA $0840
e45c: AND #$1f
e45e: ORA $4b
e460: STA $0840
e463: PLA
e464: ASL A
e465: ASL A
e466: STA $4b
e468: LDA $0840
e46b: AND #$e3
e46d: ORA $4b
e46f: STA $0840
e472: LDA $1017
e475: AND #$38
e477: LSR A
e478: LSR A
e479: LSR A
e47a: CMP #$00
e47c: STA $4b
e47e: LDA $0847
e481: AND #$f8
e483: ORA $4b
e485: STA $0847
e488: c2 !UNDEF
e489: 02 !UNDEF
e48a: RTI
e48b: PHP
e48c: CMP (,zp)$01
e48e: RTI
e48f: PHP
e490: RTS
e491: c2 !UNDEF
e492: ORA (,X)$40
e494: PHP
e495: c2 !UNDEF
e496: ORA (,X)$41
e498: PHP
e499: c2 !UNDEF
e49a: 02 !UNDEF
e49b: EOR (,X)$08
e49d: CMP (,zp)$02
e49f: RTI
e4a0: PHP
e4a1: RTS
e4a2: BBR5 $4f → $e4a5
e4a5: BBR2 $a3 → $e4a8
e4a8: CMP (,zp)$01
e4aa: TRB $d20f
e4ad: BRA $4e
e4af: BBR0 $d2 → $e4b2
e4b2: LSR $a90f
e4b5: LSR $85
e4b7: LDA $00a9
e4ba: STA $ae
e4bc: RMB1 $94
e4be: LDA #$c9
e4c0: STA $1248
e4c3: LDA #$e4
e4c5: STA $1249
e4c8: RTS
e4c9: BBR1 $94 → $e4cc
e4cc: CMP (,zp)$40
e4ce: STA ,Y$d208
e4d1: PHP
e4d2: STA ,X$d208
e4d5: PHP
e4d6: 93 !UNDEF
e4d7: PHP
e4d8: CMP (,zp)$40
e4da: ROL $0f
e4dc: CMP (,zp)$40
e4de: RMB2 $0f
e4e0: c2 !UNDEF
e4e1: JSR $0f2c
e4e4: LDA #$03
e4e6: STA $0f18
e4e9: CMP (,zp)$02
e4eb: ROL ,X$b20f
e4ee: TRB $ad
e4f0: RMB1 $94
e4f2: LDA #$fd
e4f4: STA $1248
e4f7: LDA #$e4
e4f9: STA $1249
e4fc: RTS
e4fd: BBS1 $94 → $e500
e500: RTS
e501: c2 !UNDEF
e502: PHP
e503: 93 !UNDEF
e504: PHP
e505: JSR $dab2
e508: CMP (,zp)$02
e50a: RTI
e50b: BBR0 $c7 → $e50e
e50e: RMB4 $01
e510: c2 !UNDEF
e511: ORA (,X)$c0
e513: TSB $af
e515: EOR $c20e
e518: BPL $c0
e51a: TSB $d2
e51c: ORA (,X)$c5
e51e: TSB $c2
e520: JSR $04c0
e523: BRA $1b
e525: SMB5 $9c
e527: CMP (,zp)$10
e529: CPY #$04
e52b: BBR3 $4d → $e52e
e52e: CMP (,zp)$01
e530: CMP $04
e532: CMP (,zp)$20
e534: CPY #$04
e536: BRA $08
e538: c2 !UNDEF
e539: ORA (,X)$c5
e53b: TSB $d2
e53d: JSR $04c0
e540: BBS2 $4d → $e543
e543: e2 !UNDEF
e544: ASL ,X$10
e546: JSR $ad1f
e549: ASL ,X$10
e54b: AND #$0f
e54d: ORA #$20
e54f: e2 !UNDEF
e550: ASL ,X$10
e552: BPL $02
e554: ORA #$40
e556: BBS2 $9b → $e559
e559: STA $0f2a
e55c: BBS1 $9b → $e55f
e55f: STA $0f2b
e562: LDA (,zp)$28
e564: BBS2 $27 → $e567
e567: c2 !UNDEF
e568: JSR $0f2c
e56b: CMP (,zp)$80
e56d: ASL ,X$0f
e56f: CMP (,zp)$80
e571: ORA ,X$0f
e573: RMB6 $93
e575: LDA (,zp)$32
e577: LDA $9417
e57a: RTS
e57b: JSR $1408
e57e: c2 !UNDEF
e57f: ORA (,X)$1c
e581: BBR0 $a9 → $e584
e584: STA $04f4
e587: e2 !UNDEF
e588: LSR $800f
e58b: PHA
e58c: c2 !UNDEF
e58d: 03 !UNDEF
e58e: ROL A
e58f: BBR0 $c2 → $e592
e592: 2b !UNDEF
e593: BBR0 $ad → $e596
e596: BBR0 $29 → $e599
e599: STA $0f44
e59c: c2 !UNDEF
e59d: ORA (,X)$20
e59f: BBR0 $c2 → $e5a2
e5a2: RTI
e5a3: BBR0 $c2 → $e5a6
e5a6: ROL ,X$b20f
e5a9: BVC $c1
e5ab: RMB0 $96
e5ad: JSR $a936
e5b0: BBR0 $96 → $e5b3
e5b3: JSR $e18d
e5b6: c2 !UNDEF
e5b7: RTI
e5b8: STA ,Y$c208
e5bb: PHP
e5bc: STA ,X$c208
e5bf: RTI
e5c0: LSR $c20f
e5c3: BRA $4e
e5c5: BBR0 $67 → $e5c8
e5c8: LDA #$a2
e5ca: STA $1248
e5cd: LDA #$e4
e5cf: STA $1249
e5d2: SMB6 $93
e5d4: RTS
e5d5: BBS0 $4d → $e5d8
e5d8: JSR $13cf
e5db: BBR2 $4d → $e5de
e5de: e2 !UNDEF
e5df: SMB4 $04
e5e1: PHP
e5e2: RMB3 $e2
e5e4: CPY #$04
e5e6: JSR $e232
e5e9: SMB4 $04
e5eb: JSR $d22d
e5ee: BRA $1e
e5f0: BBR0 $d2 → $e5f3
e5f3: BBR1 $0f → $e5f6
e5f6: AND (,zp)$af
e5f8: RMB2 $94
e5fa: LDA $04d8
e5fd: STA $04d8
e600: LSR A
e601: LSR A
e602: LSR A
e603: BCC $03
e605: LDA #$07
e607: RTS
e608: LSR A
e609: BCC $03
e60b: LDA #$05
e60d: RTS
e60e: LSR A
e60f: BCC $03
e611: LDA #$06
e613: RTS
e614: LSR A
e615: BCC $03
e617: LDA #$08
e619: RTS
e61a: LDA #$ff
e61c: STA $04d8
e61f: RMB5 $00
e621: BBS3 $4d → $e624
e624: SMB2 $01
e626: JSR $e66e
e629: LDA $02
e62b: RMB2 $01
e62d: BRA $07
e62f: LDA $0f7a
e632: LSR A
e633: LSR A
e634: LSR A
e635: LSR A
e636: SMB5 $00
e638: AND #$0f
e63a: CMP #$0d
e63c: BCS $15
e63e: TAX
e63f: LDA ,X$f799
e642: STA $45
e644: SMB5 $00
e646: JSR $e66e
e649: LDA $02
e64b: RMB5 $00
e64d: AND #$0f
e64f: CMP #$0d
e651: BCC $03
e653: JMP $e66b
e656: TAX
e657: LDA ,X$f78c
e65a: STA $44
e65c: LDX $44
e65e: BEQ $04
e660: LDA $45
e662: BRA $03
e664: LDA $45
e666: RTS
e667: BNE $02
e669: TXA
e66a: RTS
e66b: LDA #$00
e66d: RTS
e66e: JSR $e673
e671: BRA $00
e673: 22 !UNDEF
e674: AND (,zp)$22
e676: AND (,zp)$22
e678: AND (,zp)$ea
e67a: NOP
e67b: NOP
e67c: RTS
e67d: BBR6 $94 → $e680
e680: BBR2 $a3 → $e683
e683: SBC (,zp)$4e
e685: ASL $1001
e688: BBR1 $9d → $e68b
e68b: RMB1 $9d
e68d: JSR $e5d5
e690: CMP $1267
e693: BEQ $04
e695: STA $1267
e698: RTS
e699: BBR4 $9b → $e69c
e69c: CMP #$00
e69e: BEQ $01
e6a0: RTS
e6a1: RMB4 $9b
e6a3: LDA (,zp)$0a
e6a5: CMP #$67
e6a7: STY ,X$a9
e6d5: JSR $13d5
e6d8: BBR2 $a3 → $e6db
e6db: BBS0 $4f → $e6de
e6de: BBS1 $4f → $e6e1
e6e1: BRA $02
e6e3: AND #$fc
e6e5: CMP $127d
e6e8: BEQ $16
e6ea: STA $127d
e6ed: STA $138d
e6f0: LDA #$01
e6f2: STA $127c
e6f5: STA $138c
e6f8: LDA #$02
e6fa: STA $138b
e6fd: JSR $c9c2
e700: RTS
e701: JSR $13d8
e704: BBR2 $a3 → $e707
e707: LDA #$02
e709: STA $127e
e70c: STA $138c
e70f: LDA $127f
e712: STA $138d
e715: LDA $1280
e718: STA $138e
e71b: LDA #$03
e71d: STA $138b
e720: JSR $c9c2
e723: RTS
e724: LDY $12c7
e727: JSR $13db
e72a: BBR2 $a3 → $e72d
e72d: SBC (,zp)$4e
e72f: ASL $2901
e732: BBR7 $4f → $e735
e735: CMP $12c6
e738: BNE $0b
e73a: CPY $12c7
e73d: BEQ $1c
e73f: STY $12c7
e742: STY $138e
e745: STA $12c6
e748: STA $138d
e74b: LDA #$08
e74d: STA $127c
e750: STA $138c
e753: LDA #$08
e755: STA $138b
e758: JSR $c9c2
e75b: RTS
e75c: BBS0 $94 → $e75f
e75f: RTS
e760: LDA $1009
e763: JSR $13fc
e766: BBS1 $4f → $e769
e769: BBS2 $a3 → $e76c
e76c: BBS0 $4f → $e76f
e76f: CMP $56
e771: BEQ $1e
e773: CMP $55
e775: BNE $32
e777: LDA $100a
e77a: BMI $04
e77c: CMP #$05
e77e: BCS $03
e780: INC $100a
e783: LDA $100b
e786: BMI $04
e788: CMP #$05
e78a: BCS $1d
e78c: INC $100b
e78f: BRA $18
e791: LDA $100a
e794: BPL $04
e796: CMP #$b7
e798: BCC $03
e79a: DEC $100a
e79d: LDA $100b
e7a0: BPL $04
e7a2: CMP #$b7
e7a4: BCC $03
e7a6: DEC $100b
e7a9: LDA (,zp)$05
e7ab: ab !UNDEF
e7ac: RMB0 $94
e7ae: RTS
e7af: JSR $1405
e7b2: BBR2 $4d → $e7b5
e7b5: BBR3 $4d → $e7b8
e7b8: BBS7 $02 → $e7bb
e7bb: BBR6 $9b → $e7be
e7be: RMB6 $9b
e7c0: BRA $05
e7c2: BBS6 $9b → $e7c5
e7c5: SMB6 $9b
e7c7: RMB5 $9b
e7c9: LDA (,zp)$05
e7cb: b3 !UNDEF
e7cc: RMB4 $94
e7ce: BRA $15
e7d0: BBR4 $94 → $e7d3
e7d3: BBS5 $9b → $e7d6
e7d6: SMB5 $9b
e7d8: BBR6 $9b → $e7db
e7db: CMP (,zp)$10
e7dd: CMP $04
e7df: BRA $04
e7e1: c2 !UNDEF
e7e2: BPL $c5
e7e4: TSB $3f
e7e6: EOR $a904
e7e9: BMI $80
e7eb: 03 !UNDEF
e7ec: LDA $0f7a
e7ef: AND #$f0
e7f1: STA $40
e7f3: LDA $04c7
e7f6: AND #$08
e7f8: ORA $40
e7fa: STA $40
e7fc: LDA $9a
e7fe: AND #$f8

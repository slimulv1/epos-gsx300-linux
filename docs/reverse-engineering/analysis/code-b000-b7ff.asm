b000: .byte $53   ;?
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
c000: LDA  $f626,X      ; bd 26 f6
c003: BRA  $c02c        ; 80 27
c005: DEX               ; ca 
c006: DEX               ; ca 
c007: DEX               ; ca 
c008: BPL  $bff0        ; 10 e6
c00a: LDA  #$ff         ; a9 ff
c00c: BRA  $c02c        ; 80 1e
c00e: LDX  #$1b         ; a2 1b
c010: LDA  $f645,X      ; bd 45 f6
c013: CMP  $0e14,X      ; cd 14 0e
c016: BNE  $c025        ; d0 0d
c018: LDA  $f646,X      ; bd 46 f6
c01b: CMP  $0e15,X      ; cd 15 0e
c01e: BNE  $c025        ; d0 05
c020: LDA  $f647,X      ; bd 47 f6
c023: BRA  $c02c        ; 80 07
c025: DEX               ; ca 
c026: DEX               ; ca 
c027: DEX               ; ca 
c028: BPL  $c010        ; 10 e6
c02a: LDA  #$ff         ; a9 ff
c02c: STA  $40          ; 85 40
c02e: BPL  $c03b        ; 10 0b
c030: LDA  #$4f         ; a9 4f
c032: JSR  $bf4c        ; 20 4c bf
c035: JSR  $c307        ; 20 07 c3
c038: JMP  ($c0e5)      ; 4c e5 c0
c03b: LDA  $0f13,X      ; ad 13 0f
c03e: AND  #$0f         ; 29 0f
c040: CMP  $12d0,X      ; cd d0 12
c043: BEQ  $c062        ; f0 1d
c045: LDA  $12d0,X      ; ad d0 12
c048: CMP  $40          ; c5 40
c04a: BNE  $c062        ; d0 16
c04c: STA  $4b          ; 85 4b
c04e: LDA  $0f13,X      ; ad 13 0f
c051: AND  #$f0         ; 29 f0
c053: ORA  $4b          ; 05 4b
c055: STA  $0f13,X      ; 8d 13 0f
c058: JSR  $d8b4        ; 20 b4 d8
c05b: CMP  ($01)        ; d2 01
c05d: INY               ; c8 
c05e: ORA  ($4c),Y      ; 11 4c
c060: .byte $d9   ;?
c061: CPY  #$ad         ; c0 ad
c063: LDA  $850f,X      ; bd 0f 85
c066: .byte $42   ;?
c067: LDA  $0fbc,X      ; ad bc 0f
c06a: AND  #$07         ; 29 07
c06c: STA  $43          ; 85 43
c06e: LDA  $0fbb,X      ; ad bb 0f
c071: STA  $44          ; 85 44
c073: LDA  $0fb8,X      ; ad b8 0f
c076: AND  #$07         ; 29 07
c078: STA  $45          ; 85 45
c07a: LDA  $0fbe,X      ; ad be 0f
c07d: AND  #$c0         ; 29 c0
c07f: CMP  #$c0         ; c9 c0
c081: BNE  $c087        ; d0 04
c083: ASL  $44,X        ; 06 44
c085: ROL  $45,X        ; 26 45
c087: SBC  ($71)        ; f2 71
c089: .byte $0f   ;?
c08a: ORA  ($0e,X)      ; 01 0e
c08c: LDA  $0fc7,X      ; ad c7 0f
c08f: STA  $40          ; 85 40
c091: LDA  $0fc6,X      ; ad c6 0f
c094: AND  #$07         ; 29 07
c096: STA  $41          ; 85 41
c098: BRA  $c0a6        ; 80 0c
c09a: LDA  $0fc5,X      ; ad c5 0f
c09d: STA  $40          ; 85 40
c09f: LDA  $0fc4,X      ; ad c4 0f
c0a2: AND  #$07         ; 29 07
c0a4: STA  $41          ; 85 41
c0a6: SEC               ; 38 
c0a7: LDA  $40          ; a5 40
c0a9: SBC  $42          ; e5 42
c0ab: STA  $40          ; 85 40
c0ad: LDA  $41          ; a5 41
c0af: SBC  $43          ; e5 43
c0b1: STA  $41          ; 85 41
c0b3: BPL  $c0c1        ; 10 0c
c0b5: LDA  $40          ; a5 40
c0b7: ADC  $44          ; 65 44
c0b9: STA  $40          ; 85 40
c0bb: LDA  $41          ; a5 41
c0bd: ADC  $45          ; 65 45
c0bf: STA  $41          ; 85 41
c0c1: JSR  $1462        ; 20 62 14
c0c4: SBC  ($71)        ; f2 71
c0c6: .byte $0f   ;?
c0c7: ORA  ($10,X)      ; 01 10
c0c9: LDA  $d3          ; a5 d3
c0cb: BMI  $c0d1        ; 30 04
c0cd: DEC  $d3,X        ; c6 d3
c0cf: BPL  $c0d9        ; 10 08
c0d1: LDA  $0f71,X      ; ad 71 0f
c0d4: ORA  #$03         ; 09 03
c0d6: STA  $0f71,X      ; 8d 71 0f
c0d9: CMP  ($01)        ; d2 01
c0db: ASL  $0e,X        ; 16 0e
c0dd: .byte $e2   ;?
c0de: ASL  $0e,X        ; 16 0e
c0e0: PHP               ; 08 
c0e1: .byte $03   ;?
c0e2: JSR  $c307        ; 20 07 c3
c0e5: .byte $e2   ;?
c0e6: PHP               ; 08 
c0e7: ORA  $2504,X      ; 0d 04 25
c0ea: LDA  #$04         ; a9 04
c0ec: STA  $0d08,X      ; 8d 08 0d
c0ef: SBC  ($26)        ; f2 26
c0f1: ASL  $1b01        ; 0e 01 1b
c0f4: .byte $e2   ;?
c0f5: ROL  $0e,X        ; 26 0e
c0f7: BRA  $c103        ; 80 0a
c0f9: LDA  #$43         ; a9 43
c0fb: STA  $38          ; 85 38
c0fd: JSR  $c31b        ; 20 1b c3
c100: JMP  ($c0e5)      ; 4c e5 c0
c103: CMP  ($01)        ; d2 01
c105: ROL  $0e,X        ; 26 0e
c107: .byte $e2   ;?
c108: ROL  $0e,X        ; 26 0e
c10a: PHP               ; 08 
c10b: .byte $03   ;?
c10c: JSR  $c31b        ; 20 1b c3
c10f: RTS               ; 60 
c110: LDA  $0d0a,X      ; ad 0a 0d
c113: BNE  $c116        ; d0 01
c115: RTS               ; 60 
c116: JSR  $1432        ; 20 32 14
c119: .byte $e2   ;?
c11a: ASL               ; 0a 
c11b: ORA  $3401,X      ; 0d 01 34
c11e: LDA  #$01         ; a9 01
c120: STA  $0d0a,X      ; 8d 0a 0d
c123: .byte $e2   ;?
c124: ASL  $800e        ; 0e 0e 80
c127: ORA  #$20         ; 09 20
c129: .byte $fb   ;?
c12a: .byte $c2   ;?
c12b: CMP  ($01)        ; d2 01
c12d: ASL  $800e        ; 0e 0e 80
c130: AND  ($f2,X)      ; 21 f2
c132: ASL  $010e        ; 0e 0e 01
c135: ORA  #$87         ; 09 87
c137: LDA  ($e2,X)      ; a1 e2
c139: ASL  $040e        ; 0e 0e 04
c13c: .byte $02   ;?
c13d: .byte $07   ;?
c13e: LDA  ($ad,X)      ; a1 ad
c140: ADC  $c913,X      ; 7d 13 c9
c143: TSB  $d0          ; 04 d0
c145: TSB  ($7fad)      ; 0c ad 7f
c148: .byte $13   ;?
c149: BEQ  $c152        ; f0 07
c14b: CMP  $0d00,X      ; cd 00 0d
c14e: BEQ  $c152        ; f0 02
c150: .byte $e7   ;?
c151: LDA  ($e2,X)      ; a1 e2
c153: ASL               ; 0a 
c154: ORA  $0d02,X      ; 0d 02 0d
c157: LDA  #$02         ; a9 02
c159: STA  $0d0a,X      ; 8d 0a 0d
c15c: .byte $e2   ;?
c15d: ASL  $800e        ; 1e 0e 80
c160: ASL  $20,X        ; 06 20
c162: BIT  ($4cc3)      ; 2c c3 4c
c165: .byte $63   ;?
c166: .byte $c2   ;?
c167: SBC  ($1e)        ; f2 1e
c169: ASL  $f801        ; 0e 01 f8
c16c: LDA  #$00         ; a9 00
c16e: STA  $40          ; 85 40
c170: STA  $41          ; 85 41
c172: STA  $d4          ; 85 d4
c174: .byte $af   ;?
c175: LDX  #$03         ; a2 03
c177: JMP  ($c249)      ; 4c 49 c2
c17a: .byte $ef   ;?
c17b: .byte $a3   ;?
c17c: ORA  $e7          ; 05 e7
c17e: .byte $a3   ;?
c17f: JSR  $c2da        ; 20 da c2
c182: LDA  $0ffd,X      ; ad fd 0f
c185: STA  $40          ; 85 40
c187: LDA  $0ffc,X      ; ad fc 0f
c18a: AND  #$07         ; 29 07
c18c: STA  $41          ; 85 41
c18e: LDA  $0f80,X      ; ad 80 0f
c191: AND  #$07         ; 29 07
c193: STA  $45          ; 85 45
c195: LDA  $0f83,X      ; ad 83 0f
c198: ASL               ; 0a 
c199: STA  $44          ; 85 44
c19b: ROL  $45,X        ; 26 45
c19d: LDA  $0f86,X      ; ad 86 0f
c1a0: AND  #$c0         ; 29 c0
c1a2: CMP  #$c0         ; c9 c0
c1a4: BNE  $c1aa        ; d0 04
c1a6: ASL  $44,X        ; 06 44
c1a8: ROL  $45,X        ; 26 45
c1aa: SBC  ($70)        ; f2 70
c1ac: .byte $0f   ;?
c1ad: ORA  ($0e,X)      ; 01 0e
c1af: LDA  $0f87,X      ; ad 87 0f
c1b2: STA  $42          ; 85 42
c1b4: LDA  $0f86,X      ; ad 86 0f
c1b7: AND  #$07         ; 29 07
c1b9: STA  $43          ; 85 43
c1bb: BRA  $c1c9        ; 80 0c
c1bd: LDA  $0f85,X      ; ad 85 0f
c1c0: STA  $42          ; 85 42
c1c2: LDA  $0f84,X      ; ad 84 0f
c1c5: AND  #$07         ; 29 07
c1c7: STA  $43          ; 85 43
c1c9: SEC               ; 38 
c1ca: LDA  $40          ; a5 40
c1cc: SBC  $42          ; e5 42
c1ce: STA  $40          ; 85 40
c1d0: LDA  $41          ; a5 41
c1d2: SBC  $43          ; e5 43
c1d4: STA  $41          ; 85 41
c1d6: BPL  $c1e4        ; 10 0c
c1d8: LDA  $40          ; a5 40
c1da: ADC  $44          ; 65 44
c1dc: STA  $40          ; 85 40
c1de: LDA  $41          ; a5 41
c1e0: ADC  $45          ; 65 45
c1e2: STA  $41          ; 85 41
c1e4: JSR  $1465        ; 20 65 14
c1e7: SBC  ($70)        ; f2 70
c1e9: .byte $0f   ;?
c1ea: ORA  ($10,X)      ; 01 10
c1ec: LDA  $d5          ; a5 d5
c1ee: BMI  $c1f4        ; 30 04
c1f0: DEC  $d5,X        ; c6 d5
c1f2: BPL  $c1fc        ; 10 08
c1f4: LDA  $0f70,X      ; ad 70 0f
c1f7: ORA  #$41         ; 09 41
c1f9: STA  $0f70,X      ; 8d 70 0f
c1fc: LDA  $12d1,X      ; ad d1 12
c1ff: ASL               ; 0a 
c200: TAX               ; aa 
c201: CMP  #$0c         ; c9 0c
c203: BNE  $c20f        ; d0 0a
c205: DEC  $13b2        ; ce b2 13
c208: BNE  $c20f        ; d0 05
c20a: JSR  $c2da        ; 20 da c2
c20d: BRA  $c22d        ; 80 1e
c20f: LDA  $d7          ; a5 d7
c211: CMP  #$01         ; c9 01
c213: BEQ  $c221        ; f0 0c
c215: LDA  $f693,X      ; bd 93 f6
c218: STA  $40          ; 85 40
c21a: LDA  $f694,X      ; bd 94 f6
c21d: STA  $41          ; 85 41
c21f: BRA  $c249        ; 80 28
c221: LDA  $f663,X      ; bd 63 f6
c224: STA  $40          ; 85 40
c226: LDA  $f664,X      ; bd 64 f6
c229: STA  $41          ; 85 41
c22b: BRA  $c249        ; 80 1c
c22d: LDA  $d7          ; a5 d7
c22f: CMP  #$01         ; c9 01
c231: BEQ  $c23f        ; f0 0c
c233: LDA  $f6a3,X      ; bd a3 f6
c236: STA  $40          ; 85 40
c238: LDA  $f6a4,X      ; bd a4 f6
c23b: STA  $41          ; 85 41
c23d: BRA  $c249        ; 80 0a
c23f: LDA  $f673,X      ; bd 73 f6
c242: STA  $40          ; 85 40
c244: LDA  $f674,X      ; bd 74 f6
c247: STA  $41          ; 85 41
c249: .byte $1f   ;?
c24a: EOR  $1805,X      ; 4d 05 18
c24d: LSR  $41,X        ; 46 41
c24f: ROR  $40,X        ; 66 40
c251: LDA  $40          ; a5 40
c253: STA  $0e1c,X      ; 8d 1c 0e
c256: LDA  $41          ; a5 41
c258: STA  $0e1d,X      ; 8d 1d 0e
c25b: .byte $c2   ;?
c25c: TSB  $1e          ; 04 1e
c25e: ASL  $01d2        ; 0e d2 01
c261: ASL  $e20e        ; 1e 0e e2
c264: ASL               ; 0a 
c265: ORA  $0504,X      ; 0d 04 05
c268: LDA  #$04         ; a9 04
c26a: STA  $0d0a,X      ; 8d 0a 0d
c26d: .byte $e2   ;?
c26e: ASL               ; 0a 
c26f: ORA  $0508,X      ; 0d 08 05
c272: LDA  #$08         ; a9 08
c274: STA  $0d0a,X      ; 8d 0a 0d
c277: .byte $e2   ;?
c278: ASL               ; 0a 
c279: ORA  $1410,X      ; 0d 10 14
c27c: LDA  #$10         ; a9 10
c27e: STA  $0d0a,X      ; 8d 0a 0d
c281: .byte $e2   ;?
c282: LSR  $800e        ; 4e 0e 80
c285: ORA  $20          ; 05 20
c287: SEC               ; 38 
c288: .byte $c3   ;?
c289: BRA  $c277        ; 80 ec
c28b: SBC  ($4e)        ; f2 4e
c28d: ASL  $0001        ; 0e 01 00
c290: RTS               ; 60 
c291: LDA  $41          ; a5 41
c293: CMP  $027a,X      ; cd 7a 02
c296: BNE  $c29d        ; d0 05
c298: LDA  $40          ; a5 40
c29a: CMP  $0279,X      ; cd 79 02
c29d: BEQ  $c2ae        ; f0 0f
c29f: LDA  $40          ; a5 40
c2a1: STA  $0279,X      ; 8d 79 02
c2a4: LDA  $41          ; a5 41
c2a6: STA  $027a,X      ; 8d 7a 02
c2a9: .byte $1f   ;?
c2aa: STZ  ($8002)      ; 9c 02 80
c2ad: .byte $1f   ;?
c2ae: RTS               ; 60 
c2af: LDA  $41          ; a5 41
c2b1: CMP  $0278,X      ; cd 78 02
c2b4: BNE  $c2bb        ; d0 05
c2b6: LDA  $40          ; a5 40
c2b8: CMP  $0277,X      ; cd 77 02
c2bb: BEQ  $c2cc        ; f0 0f
c2bd: LDA  $40          ; a5 40
c2bf: STA  $0277,X      ; 8d 77 02
c2c2: LDA  $41          ; a5 41
c2c4: STA  $0278,X      ; 8d 78 02
c2c7: .byte $0f   ;?
c2c8: STZ  ($8002)      ; 9c 02 80
c2cb: ORA  ($60,X)      ; 01 60
c2cd: LDA  $41          ; a5 41
c2cf: JSR  $a5c9        ; 20 c9 a5
c2d2: LDA  $40          ; a5 40
c2d4: JSR  $a5c9        ; 20 c9 a5
c2d7: JMP  ($a5c2)      ; 4c c2 a5
c2da: LDA  #$0a         ; a9 0a
c2dc: LDY  ($12d1)      ; ac d1 12
c2df: CPY  #$08         ; c0 08
c2e1: BNE  $c2e5        ; d0 02
c2e3: LDA  #$05         ; a9 05
c2e5: STA  $13b2,X      ; 8d b2 13
c2e8: RTS               ; 60 
c2e9: .byte $37   ;?
c2ea: .byte $9f   ;?
c2eb: CMP  ($01)        ; d2 01
c2ed: ASL  $0e,X        ; 06 0e
c2ef: CMP  ($10)        ; d2 10
c2f1: ASL  $0e,X        ; 06 0e
c2f3: CMP  ($02)        ; d2 02
c2f5: ASL  $0e,X        ; 06 0e
c2f7: LDA  #$40         ; a9 40
c2f9: BRA  $c342        ; 80 47
c2fb: CMP  ($10)        ; d2 10
c2fd: ASL  $d20e        ; 0e 0e d2
c300: .byte $02   ;?
c301: ASL  $a90e        ; 0e 0e a9
c304: EOR  ($80,X)      ; 41 80
c306: .byte $3b   ;?
c307: JSR  $1417        ; 20 17 14
c30a: .byte $17   ;?
c30b: LDX  #$20         ; a2 20
c30d: LDY  $d8,X        ; b4 d8
c30f: .byte $c2   ;?
c310: BRA  $c328        ; 80 16
c312: ASL  $01d2        ; 0e d2 01
c315: ASL  $0e,X        ; 16 0e
c317: LDA  #$45         ; a9 45
c319: BRA  $c342        ; 80 27
c31b: JSR  $141a        ; 20 1a 14
c31e: .byte $07   ;?
c31f: LDX  #$c2         ; a2 c2
c321: BRA  $c349        ; 80 26
c323: ASL  $01d2        ; 0e d2 01
c326: ROL  $0e,X        ; 26 0e
c328: LDA  #$43         ; a9 43
c32a: BRA  $c342        ; 80 16
c32c: CMP  ($10)        ; d2 10
c32e: ASL  $d20e        ; 1e 0e d2
c331: ORA  ($1e,X)      ; 01 1e
c333: ASL  $42a9        ; 0e a9 42
c336: BRA  $c342        ; 80 0a
c338: CMP  ($10)        ; d2 10
c33a: LSR  $d20e        ; 4e 0e d2
c33d: .byte $02   ;?
c33e: LSR  $a90e        ; 4e 0e a9
c341: LSR  $60,X        ; 46 60
c343: LDA  $0e04,X      ; ad 04 0e
c346: BNE  $c34c        ; d0 04
c348: .byte $07   ;?
c349: LDY  #$80         ; a0 80
c34b: PLA               ; 68 
c34c: STA  $44          ; 85 44
c34e: LDA  ($fd)        ; b2 fd
c350: RTI               ; 40 
c351: LDA  ($12)        ; b2 12
c353: EOR  ($07,X)      ; 41 07
c355: LDY  #$20         ; a0 20
c357: BRK               ; 00 
c358: DEX               ; ca 
c359: .byte $0f   ;?
c35a: .byte $9f   ;?
c35b: ASL               ; 0a 
c35c: CMP  ($10)        ; d2 10
c35e: ASL  $d20e        ; 0e 0e d2
c361: .byte $02   ;?
c362: ASL  $800e        ; 0e 0e 80
c365: LSR  $9f1f        ; 4e 1f 9f
c368: TSB  $87          ; 04 87
c36a: LDY  #$80         ; a0 80
c36c: .byte $47   ;?
c36d: .byte $2f   ;?
c36e: .byte $9f   ;?
c36f: ORA  $20          ; 05 20
c371: ROR  $c9,X        ; 66 c9
c373: BRA  $c3b4        ; 80 3f
c375: LDA  $40          ; a5 40
c377: STA  $1380,X      ; 8d 80 13
c37a: LDA  $41          ; a5 41
c37c: STA  $1381,X      ; 8d 81 13
c37f: LDA  $47          ; a5 47
c381: CMP  $45          ; c5 45
c383: BNE  $c389        ; d0 04
c385: LDA  $46          ; a5 46
c387: CMP  $44          ; c5 44
c389: BCC  $c397        ; 90 0c
c38b: LDA  $45          ; a5 45
c38d: STA  $1383,X      ; 8d 83 13
c390: LDA  $44          ; a5 44
c392: STA  $1382,X      ; 8d 82 13
c395: BRA  $c3a1        ; 80 0a
c397: LDA  $47          ; a5 47
c399: STA  $1383,X      ; 8d 83 13
c39c: LDA  $46          ; a5 46
c39e: STA  $1382,X      ; 8d 82 13
c3a1: .byte $87   ;?
c3a2: LDA  ($37,X)      ; a1 37
c3a4: .byte $9f   ;?
c3a5: LDA  #$bb         ; a9 bb
c3a7: STA  $124e,X      ; 8d 4e 12
c3aa: LDA  #$c3         ; a9 c3
c3ac: STA  $124f,X      ; 8d 4f 12
c3af: .byte $b7   ;?
c3b0: .byte $9f   ;?
c3b1: JMP  ($124d)      ; 4c 4d 12
c3b4: .byte $37   ;?
c3b5: .byte $9f   ;?
c3b6: CMP  ($01)        ; d2 01
c3b8: ASL  $0e,X        ; 06 0e
c3ba: RTS               ; 60 
c3bb: .byte $e2   ;?
c3bc: ASL  $010e        ; 0e 0e 01
c3bf: ORA  ($60,X)      ; 01 60
c3c1: LDA  $1380,X      ; ad 80 13
c3c4: STA  $40          ; 85 40
c3c6: LDA  $1381,X      ; ad 81 13
c3c9: STA  $41          ; 85 41
c3cb: LDA  $1383,X      ; ad 83 13
c3ce: BNE  $c3d9        ; d0 09
c3d0: LDA  $1382,X      ; ad 82 13
c3d3: CMP  #$40         ; c9 40
c3d5: BCC  $c3db        ; 90 04
c3d7: BEQ  $c3db        ; f0 02
c3d9: LDA  #$40         ; a9 40
c3db: STA  $45          ; 85 45
c3dd: LDX  #$00         ; a2 00
c3df: .byte $1f   ;?
c3e0: EOR  $a528,X      ; 4d 28 a5
c3e3: EOR  ($c9,X)      ; 41 c9
c3e5: SBC  $04d0,X      ; ed d0 04
c3e8: LDA  $40          ; a5 40
c3ea: CMP  #$02         ; c9 02
c3ec: BNE  $c3f6        ; d0 08
c3ee: LDA  ($04)        ; b2 04
c3f0: RTI               ; 40 
c3f1: LDA  ($ed)        ; b2 ed
c3f3: EOR  ($80,X)      ; 41 80
c3f5: SBC  #$a5         ; e9 a5
c3f7: EOR  ($c9,X)      ; 41 c9
c3f9: SBC  $04d0,X      ; ed d0 04
c3fc: LDA  $40          ; a5 40
c3fe: CMP  #$1d         ; c9 1d
c400: BNE  $c40a        ; d0 08
c402: LDA  ($1f)        ; b2 1f
c404: RTI               ; 40 
c405: LDA  ($ed)        ; b2 ed
c407: EOR  ($80,X)      ; 41 80
c409: CMP  $9f          ; d5 9f
c40b: .byte $4f   ;?
c40c: .byte $03   ;?
c40d: .byte $0f   ;?
c40e: .byte $4f   ;?
c40f: ORA  $41a5,X      ; 1d a5 41
c412: CMP  #$ee         ; c9 ee
c414: BNE  $c41a        ; d0 04
c416: LDA  $40          ; a5 40
c418: CMP  #$de         ; c9 de
c41a: BEQ  $c428        ; f0 0c
c41c: LDA  $41          ; a5 41
c41e: CMP  #$ee         ; c9 ee
c420: BNE  $c426        ; d0 04
c422: LDA  $40          ; a5 40
c424: CMP  #$e0         ; c9 e0
c426: BNE  $c42d        ; d0 05
c428: LDA  #$00         ; a9 00
c42a: JMP  ($c6fd)      ; 4c fd c6
c42d: LDA  $41          ; a5 41
c42f: CMP  #$ee         ; c9 ee
c431: BNE  $c437        ; d0 04
c433: LDA  $40          ; a5 40
c435: CMP  #$8c         ; c9 8c
c437: BNE  $c441        ; d0 08
c439: .byte $7f   ;?
c43a: .byte $4f   ;?
c43b: .byte $19   ;?
c43c: LDA  #$3a         ; a9 3a
c43e: JMP  ($c6fd)      ; 4c fd c6
c441: LDA  $41          ; a5 41
c443: CMP  #$ee         ; c9 ee
c445: BNE  $c44b        ; d0 04
c447: LDA  $40          ; a5 40
c449: CMP  #$8d         ; c9 8d
c44b: BNE  $c455        ; d0 08
c44d: .byte $7f   ;?
c44e: .byte $4f   ;?
c44f: ORA  $a9          ; 05 a9
c451: .byte $02   ;?
c452: JMP  ($c6fd)      ; 4c fd c6
c455: LDA  $41          ; a5 41
c457: CMP  #$ec         ; c9 ec
c459: BNE  $c45f        ; d0 04
c45b: LDA  $40          ; a5 40
c45d: CMP  #$da         ; c9 da
c45f: BNE  $c470        ; d0 0f
c461: LDA  ($40,X)      ; a1 40
c463: .byte $5f   ;?
c464: .byte $4f   ;?
c465: .byte $02   ;?
c466: AND  #$bf         ; 29 bf
c468: .byte $6f   ;?
c469: .byte $4f   ;?
c46a: .byte $02   ;?
c46b: ORA  #$20         ; 09 20
c46d: JMP  ($c6fd)      ; 4c fd c6
c470: .byte $5f   ;?
c471: .byte $4f   ;?
c472: ORA  ($a5),Y      ; 11 a5
c474: EOR  ($c9,X)      ; 41 c9
c476: CPX  ($04d0)      ; ec d0 04
c479: LDA  $40          ; a5 40
c47b: CMP  #$db         ; c9 db
c47d: BNE  $c484        ; d0 05
c47f: LDA  $88          ; a5 88
c481: JMP  ($c6fd)      ; 4c fd c6
c484: LDA  $41          ; a5 41
c486: CMP  #$ec         ; c9 ec
c488: BNE  $c48e        ; d0 04
c48a: LDA  $40          ; a5 40
c48c: CMP  #$d5         ; c9 d5
c48e: BEQ  $c4ca        ; f0 3a
c490: LDA  $41          ; a5 41
c492: CMP  #$ec         ; c9 ec
c494: BNE  $c49a        ; d0 04
c496: LDA  $40          ; a5 40
c498: CMP  #$d6         ; c9 d6
c49a: BEQ  $c4ea        ; f0 4e
c49c: LDA  $41          ; a5 41
c49e: CMP  #$ec         ; c9 ec
c4a0: BNE  $c4a6        ; d0 04
c4a2: LDA  $40          ; a5 40
c4a4: CMP  #$d7         ; c9 d7
c4a6: BNE  $c4ab        ; d0 03
c4a8: JMP  ($c513)      ; 4c 13 c5
c4ab: JSR  $c744        ; 20 44 c7
c4ae: BCC  $c4b3        ; 90 03
c4b0: JMP  ($c508)      ; 4c 08 c5
c4b3: JSR  $c853        ; 20 53 c8
c4b6: BCC  $c4bb        ; 90 03
c4b8: JMP  ($c508)      ; 4c 08 c5
c4bb: .byte $4f   ;?
c4bc: .byte $4f   ;?
c4bd: ORA  #$af         ; 09 af
c4bf: .byte $4f   ;?
c4c0: ASL  $bf,X        ; 06 bf
c4c2: .byte $4f   ;?
c4c3: .byte $03   ;?
c4c4: JMP  ($c6e9)      ; 4c e9 c6
c4c7: JMP  ($c523)      ; 4c 23 c5
c4ca: .byte $2f   ;?
c4cb: .byte $4f   ;?
c4cc: TSB  $a9          ; 04 a9
c4ce: INX               ; e8 
c4cf: BRA  $c50b        ; 80 3a
c4d1: .byte $3f   ;?
c4d2: .byte $4f   ;?
c4d3: .byte $0b   ;?
c4d4: .byte $cf   ;?
c4d5: .byte $4f   ;?
c4d6: TSB  $a9          ; 04 a9
c4d8: ASL               ; 0a 
c4d9: BRA  $c508        ; 80 2d
c4db: LDA  #$2d         ; a9 2d
c4dd: BRA  $c508        ; 80 29
c4df: .byte $cf   ;?
c4e0: .byte $4f   ;?
c4e1: TSB  $a9          ; 04 a9
c4e3: .byte $ab   ;?
c4e4: BRA  $c50b        ; 80 25
c4e6: LDA  #$c2         ; a9 c2
c4e8: BRA  $c50b        ; 80 21
c4ea: .byte $2f   ;?
c4eb: .byte $4f   ;?
c4ec: TSB  $a9          ; 04 a9
c4ee: BRK               ; 00 
c4ef: BRA  $c508        ; 80 17
c4f1: .byte $3f   ;?
c4f2: .byte $4f   ;?
c4f3: .byte $0b   ;?
c4f4: .byte $cf   ;?
c4f5: .byte $4f   ;?
c4f6: TSB  $a9          ; 04 a9
c4f8: ORA  ($80,X)      ; 01 80
c4fa: ORA  $01a9,X      ; 0d a9 01
c4fd: BRA  $c508        ; 80 09
c4ff: .byte $cf   ;?
c500: .byte $4f   ;?
c501: TSB  $a9          ; 04 a9
c503: ORA  ($80,X)      ; 01 80
c505: .byte $02   ;?
c506: LDA  #$01         ; a9 01
c508: JMP  ($c6fd)      ; 4c fd c6
c50b: .byte $1f   ;?
c50c: EOR  $38fa,X      ; 4d fa 38
c50f: SBC  #$04         ; e9 04
c511: BRA  $c508        ; 80 f5
c513: LDA  ($40,X)      ; a1 40
c515: SEC               ; 38 
c516: .byte $af   ;?
c517: .byte $4f   ;?
c518: ORA  $bf          ; 05 bf
c51a: .byte $4f   ;?
c51b: .byte $02   ;?
c51c: BRA  $c520        ; 80 02
c51e: SBC  #$01         ; e9 01
c520: JMP  ($c6fd)      ; 4c fd c6
c523: LDA  $41          ; a5 41
c525: CMP  #$ee         ; c9 ee
c527: BNE  $c52d        ; d0 04
c529: LDA  $40          ; a5 40
c52b: CMP  #$7e         ; c9 7e
c52d: BNE  $c53b        ; d0 0c
c52f: .byte $af   ;?
c530: .byte $4f   ;?
c531: ORA  $bf          ; 05 bf
c533: .byte $4f   ;?
c534: .byte $02   ;?
c535: BRA  $c53b        ; 80 04
c537: LDA  #$02         ; a9 02
c539: BRA  $c508        ; 80 cd
c53b: LDA  $41          ; a5 41
c53d: CMP  #$ec         ; c9 ec
c53f: BNE  $c545        ; d0 04
c541: LDA  $40          ; a5 40
c543: CMP  #$e5         ; c9 e5
c545: BNE  $c553        ; d0 0c
c547: .byte $af   ;?
c548: .byte $4f   ;?
c549: ORA  $bf          ; 05 bf
c54b: .byte $4f   ;?
c54c: .byte $02   ;?
c54d: BRA  $c553        ; 80 04
c54f: LDA  #$09         ; a9 09
c551: BRA  $c520        ; 80 cd
c553: LDA  $41          ; a5 41
c555: CMP  #$ec         ; c9 ec
c557: BNE  $c55d        ; d0 04
c559: LDA  $40          ; a5 40
c55b: CMP  #$ea         ; c9 ea
c55d: BNE  $c586        ; d0 27
c55f: .byte $2f   ;?
c560: .byte $4f   ;?
c561: TSB  $a9          ; 04 a9
c563: LSR  $80,X        ; 46 80
c565: .byte $17   ;?
c566: .byte $3f   ;?
c567: .byte $4f   ;?
c568: .byte $0b   ;?
c569: .byte $cf   ;?
c56a: .byte $4f   ;?
c56b: TSB  $a9          ; 04 a9
c56d: PLP               ; 28 
c56e: BRA  $c520        ; 80 b0
c570: LDA  #$4b         ; a9 4b
c572: BRA  $c520        ; 80 ac
c574: .byte $cf   ;?
c575: .byte $4f   ;?
c576: TSB  $a9          ; 04 a9
c578: BIT  #$80         ; 89 80
c57a: .byte $02   ;?
c57b: LDA  #$a0         ; a9 a0
c57d: .byte $1f   ;?
c57e: EOR  $3803,X      ; 4d 03 38
c581: SBC  #$04         ; e9 04
c583: JMP  ($c520)      ; 4c 20 c5
c586: LDA  $41          ; a5 41
c588: CMP  #$ec         ; c9 ec
c58a: BNE  $c590        ; d0 04
c58c: LDA  $40          ; a5 40
c58e: CMP  #$ec         ; c9 ec
c590: BNE  $c59f        ; d0 0d
c592: .byte $af   ;?
c593: .byte $4f   ;?
c594: ORA  $bf          ; 05 bf
c596: .byte $4f   ;?
c597: .byte $02   ;?
c598: BRA  $c59f        ; 80 05
c59a: LDA  #$01         ; a9 01
c59c: JMP  ($c6fd)      ; 4c fd c6
c59f: LDA  $41          ; a5 41
c5a1: CMP  #$ec         ; c9 ec
c5a3: BNE  $c5a9        ; d0 04
c5a5: LDA  $40          ; a5 40
c5a7: CMP  #$ee         ; c9 ee
c5a9: BNE  $c5c3        ; d0 18
c5ab: .byte $2f   ;?
c5ac: .byte $4f   ;?
c5ad: ORA  #$b2         ; 09 b2
c5af: .byte $ef   ;?
c5b0: RTI               ; 40 
c5b1: LDA  ($ec)        ; b2 ec
c5b3: EOR  ($4c,X)      ; 41 4c
c5b5: .byte $df   ;?
c5b6: .byte $c3   ;?
c5b7: .byte $3f   ;?
c5b8: .byte $4f   ;?
c5b9: ORA  #$b2         ; 09 b2
c5bb: BIT  ($b240)      ; 2c 40 b2
c5be: SBC  $4c41,X      ; ed 41 4c
c5c1: .byte $df   ;?
c5c2: .byte $c3   ;?
c5c3: .byte $2f   ;?
c5c4: .byte $4f   ;?
c5c5: AND  $41a5,X      ; 2d a5 41
c5c8: CMP  #$ed         ; c9 ed
c5ca: BNE  $c5d0        ; d0 04
c5cc: LDA  $40          ; a5 40
c5ce: CMP  #$2c         ; c9 2c
c5d0: BNE  $c5db        ; d0 09
c5d2: LDA  ($85)        ; b2 85
c5d4: RTI               ; 40 
c5d5: LDA  ($ed)        ; b2 ed
c5d7: EOR  ($4c,X)      ; 41 4c
c5d9: .byte $df   ;?
c5da: .byte $c3   ;?
c5db: LDA  $41          ; a5 41
c5dd: CMP  #$ed         ; c9 ed
c5df: BNE  $c5e5        ; d0 04
c5e1: LDA  $40          ; a5 40
c5e3: CMP  #$fc         ; c9 fc
c5e5: BEQ  $c5ea        ; f0 03
c5e7: JMP  ($c6e9)      ; 4c e9 c6
c5ea: LDA  ($7c)        ; b2 7c
c5ec: RTI               ; 40 
c5ed: LDA  ($ee)        ; b2 ee
c5ef: EOR  ($4c,X)      ; 41 4c
c5f1: .byte $df   ;?
c5f2: .byte $c3   ;?
c5f3: LDA  $41          ; a5 41
c5f5: CMP  #$ed         ; c9 ed
c5f7: BNE  $c5fd        ; d0 04
c5f9: LDA  $40          ; a5 40
c5fb: CMP  #$3c         ; c9 3c
c5fd: BNE  $c60a        ; d0 0b
c5ff: .byte $3f   ;?
c600: .byte $4f   ;?
c601: PHP               ; 08 
c602: .byte $cf   ;?
c603: .byte $4f   ;?
c604: ORA  $a9          ; 05 a9
c606: TSB  $4c          ; 04 4c
c608: SBC  $a5c6,X      ; fd c6 a5
c60b: EOR  ($c9,X)      ; 41 c9
c60d: SBC  $04d0,X      ; ed d0 04
c610: LDA  $40          ; a5 40
c612: CMP  #$4b         ; c9 4b
c614: BNE  $c622        ; d0 0c
c616: .byte $3f   ;?
c617: .byte $4f   ;?
c618: ORA  #$b2         ; 09 b2
c61a: ADC  ($40,X)      ; 61 40
c61c: LDA  ($ed)        ; b2 ed
c61e: EOR  ($4c,X)      ; 41 4c
c620: .byte $df   ;?
c621: .byte $c3   ;?
c622: LDA  $41          ; a5 41
c624: CMP  #$ed         ; c9 ed
c626: BNE  $c62c        ; d0 04
c628: LDA  $40          ; a5 40
c62a: CMP  #$85         ; c9 85
c62c: BNE  $c63a        ; d0 0c
c62e: .byte $3f   ;?
c62f: .byte $4f   ;?
c630: ORA  #$b2         ; 09 b2
c632: CPX  ($b240)      ; fc 40 b2
c635: SBC  $4c41,X      ; ed 41 4c
c638: .byte $df   ;?
c639: .byte $c3   ;?
c63a: LDA  $41          ; a5 41
c63c: CMP  #$ed         ; c9 ed
c63e: BNE  $c644        ; d0 04
c640: LDA  $40          ; a5 40
c642: CMP  #$61         ; c9 61
c644: BNE  $c652        ; d0 0c
c646: .byte $cf   ;?
c647: .byte $4f   ;?
c648: ORA  #$b2         ; 09 b2
c64a: .byte $77   ;?
c64b: RTI               ; 40 
c64c: LDA  ($ed)        ; b2 ed
c64e: EOR  ($4c,X)      ; 41 4c
c650: .byte $df   ;?
c651: .byte $c3   ;?
c652: LDA  $41          ; a5 41
c654: CMP  #$ed         ; c9 ed
c656: BNE  $c65c        ; d0 04
c658: LDA  $40          ; a5 40
c65a: CMP  #$77         ; c9 77
c65c: BNE  $c675        ; d0 17
c65e: .byte $cf   ;?
c65f: .byte $4f   ;?
c660: TSB  ($4f3f)      ; 0c 3f 4f
c663: TSB  ($85b2)      ; 0c b2 85
c666: RTI               ; 40 
c667: LDA  ($ed)        ; b2 ed
c669: EOR  ($4c,X)      ; 41 4c
c66b: .byte $df   ;?
c66c: .byte $c3   ;?
c66d: .byte $3f   ;?
c66e: .byte $4f   ;?
c66f: ORA  $a9          ; 05 a9
c671: ORA  $fd4c,X      ; 0d 4c fd
c674: DEC  $a5,X        ; c6 a5
c676: EOR  ($c9,X)      ; 41 c9
c678: SBC  $04d0,X      ; ed d0 04
c67b: LDA  $40          ; a5 40
c67d: CMP  #$7b         ; c9 7b
c67f: BNE  $c68c        ; d0 0b
c681: .byte $4f   ;?
c682: .byte $4f   ;?
c683: .byte $03   ;?
c684: .byte $3f   ;?
c685: .byte $4f   ;?
c686: ORA  $a9          ; 05 a9
c688: .byte $02   ;?
c689: JMP  ($c6fd)      ; 4c fd c6
c68c: .byte $3f   ;?
c68d: .byte $4f   ;?
c68e: ORA  $a5          ; 15 a5
c690: EOR  ($c9,X)      ; 41 c9
c692: SBC  $04d0,X      ; ed d0 04
c695: LDA  $40          ; a5 40
c697: CMP  #$7d         ; c9 7d
c699: BNE  $c6a4        ; d0 09
c69b: LDA  ($7e)        ; b2 7e
c69d: RTI               ; 40 
c69e: LDA  ($ed)        ; b2 ed
c6a0: EOR  ($4c,X)      ; 41 4c
c6a2: .byte $df   ;?
c6a3: .byte $c3   ;?
c6a4: .byte $cf   ;?
c6a5: .byte $4f   ;?
c6a6: ORA  $a5          ; 15 a5
c6a8: EOR  ($c9,X)      ; 41 c9
c6aa: SBC  $04d0,X      ; ed d0 04
c6ad: LDA  $40          ; a5 40
c6af: CMP  #$7e         ; c9 7e
c6b1: BNE  $c6bc        ; d0 09
c6b3: LDA  ($7f)        ; b2 7f
c6b5: RTI               ; 40 
c6b6: LDA  ($ed)        ; b2 ed
c6b8: EOR  ($4c,X)      ; 41 4c
c6ba: .byte $df   ;?
c6bb: .byte $c3   ;?
c6bc: .byte $3f   ;?
c6bd: .byte $4f   ;?
c6be: ROL               ; 2a 
c6bf: LDA  $41          ; a5 41
c6c1: CMP  #$ed         ; c9 ed
c6c3: BNE  $c6c9        ; d0 04
c6c5: LDA  $40          ; a5 40
c6c7: CMP  #$fe         ; c9 fe
c6c9: BEQ  $c6e5        ; f0 1a
c6cb: LDA  $41          ; a5 41
c6cd: CMP  #$ee         ; c9 ee
c6cf: BNE  $c6d5        ; d0 04
c6d1: LDA  $40          ; a5 40
c6d3: CMP  #$07         ; c9 07
c6d5: BEQ  $c6e5        ; f0 0e
c6d7: LDA  $41          ; a5 41
c6d9: CMP  #$ee         ; c9 ee
c6db: BNE  $c6e1        ; d0 04
c6dd: LDA  $40          ; a5 40
c6df: CMP  #$44         ; c9 44
c6e1: BEQ  $c6e5        ; f0 02
c6e3: BRA  $c6e9        ; 80 04
c6e5: LDA  #$01         ; a9 01
c6e7: BRA  $c6fd        ; 80 14
c6e9: JSR  $c930        ; 20 30 c9
c6ec: BCS  $c6fd        ; b0 0f
c6ee: JSR  $1435        ; 20 35 14
c6f1: .byte $6f   ;?
c6f2: TYA               ; 98 
c6f3: .byte $07   ;?
c6f4: PHX               ; da 
c6f5: JSR  $eb95        ; 20 95 eb
c6f8: PLX               ; fa 
c6f9: BRA  $c6fd        ; 80 02
c6fb: LDA  ($40,X)      ; a1 40
c6fd: STA  $133d,X      ; 9d 3d 13
c700: INC  $40,X        ; e6 40
c702: BNE  $c706        ; d0 02
c704: INC  $41,X        ; e6 41
c706: INX               ; e8 
c707: CPX  $45          ; e4 45
c709: BCS  $c70e        ; b0 03
c70b: JMP  ($c3df)      ; 4c df c3
c70e: STX  $0e0c        ; 8e 0c 0e
c711: LDA  $1382,X      ; ad 82 13
c714: SEC               ; 38 
c715: SBC  $45          ; e5 45
c717: STA  $1382,X      ; 8d 82 13
c71a: LDA  $1383,X      ; ad 83 13
c71d: SBC  #$00         ; e9 00
c71f: STA  $1383,X      ; 8d 83 13
c722: BNE  $c736        ; d0 12
c724: LDA  $1382,X      ; ad 82 13
c727: BNE  $c736        ; d0 0d
c729: JSR  $c956        ; 20 56 c9
c72c: .byte $37   ;?
c72d: .byte $9f   ;?
c72e: CMP  ($01)        ; d2 01
c730: ASL  $0e,X        ; 06 0e
c732: .byte $67   ;?
c733: TYA               ; 98 
c734: BRA  $c743        ; 80 0d
c736: LDA  $40          ; a5 40
c738: STA  $1380,X      ; 8d 80 13
c73b: LDA  $41          ; a5 41
c73d: STA  $1381,X      ; 8d 81 13
c740: JSR  $c956        ; 20 56 c9
c743: RTS               ; 60 
c744: LDA  $41          ; a5 41
c746: CMP  #$ed         ; c9 ed
c748: BNE  $c74e        ; d0 04
c74a: LDA  $40          ; a5 40
c74c: CMP  #$46         ; c9 46
c74e: BNE  $c755        ; d0 05
c750: LDA  $89          ; a5 89
c752: JMP  ($c84f)      ; 4c 4f c8
c755: LDA  $41          ; a5 41
c757: CMP  #$ed         ; c9 ed
c759: BNE  $c75f        ; d0 04
c75b: LDA  $40          ; a5 40
c75d: CMP  #$47         ; c9 47
c75f: BNE  $c766        ; d0 05
c761: LDA  $8a          ; a5 8a
c763: JMP  ($c84f)      ; 4c 4f c8
c766: LDA  $41          ; a5 41
c768: CMP  #$ed         ; c9 ed
c76a: BNE  $c770        ; d0 04
c76c: LDA  $40          ; a5 40
c76e: CMP  #$48         ; c9 48
c770: BNE  $c777        ; d0 05
c772: LDA  $8b          ; a5 8b
c774: JMP  ($c84f)      ; 4c 4f c8
c777: LDA  $41          ; a5 41
c779: CMP  #$ec         ; c9 ec
c77b: BNE  $c781        ; d0 04
c77d: LDA  $40          ; a5 40
c77f: CMP  #$f3         ; c9 f3
c781: BNE  $c788        ; d0 05
c783: LDA  $8f          ; a5 8f
c785: JMP  ($c84f)      ; 4c 4f c8
c788: LDA  $41          ; a5 41
c78a: CMP  #$ec         ; c9 ec
c78c: BNE  $c792        ; d0 04
c78e: LDA  $40          ; a5 40
c790: CMP  #$f4         ; c9 f4
c792: BNE  $c799        ; d0 05
c794: LDA  $90          ; a5 90
c796: JMP  ($c84f)      ; 4c 4f c8
c799: LDA  $41          ; a5 41
c79b: CMP  #$ec         ; c9 ec
c79d: BNE  $c7a3        ; d0 04
c79f: LDA  $40          ; a5 40
c7a1: CMP  #$f5         ; c9 f5
c7a3: BNE  $c7aa        ; d0 05
c7a5: LDA  $91          ; a5 91
c7a7: JMP  ($c84f)      ; 4c 4f c8
c7aa: .byte $9f   ;?
c7ab: EOR  $4c03,X      ; 4d 03 4c
c7ae: EOR  ($c8),Y      ; 51 c8
c7b0: LDA  $41          ; a5 41
c7b2: CMP  #$ec         ; c9 ec
c7b4: BNE  $c7ba        ; d0 04
c7b6: LDA  $40          ; a5 40
c7b8: CMP  #$f6         ; c9 f6
c7ba: BNE  $c7c1        ; d0 05
c7bc: LDA  #$01         ; a9 01
c7be: JMP  ($c84f)      ; 4c 4f c8
c7c1: LDA  $41          ; a5 41
c7c3: CMP  #$ec         ; c9 ec
c7c5: BNE  $c7cb        ; d0 04
c7c7: LDA  $40          ; a5 40
c7c9: CMP  #$f7         ; c9 f7
c7cb: BNE  $c7d1        ; d0 04
c7cd: LDA  #$00         ; a9 00
c7cf: BRA  $c84f        ; 80 7e
c7d1: LDA  $41          ; a5 41
c7d3: CMP  #$ec         ; c9 ec
c7d5: BNE  $c7db        ; d0 04
c7d7: LDA  $40          ; a5 40
c7d9: CMP  #$fb         ; c9 fb
c7db: BNE  $c7e1        ; d0 04
c7dd: LDA  #$08         ; a9 08
c7df: BRA  $c84f        ; 80 6e
c7e1: LDA  $41          ; a5 41
c7e3: CMP  #$ed         ; c9 ed
c7e5: BNE  $c7eb        ; d0 04
c7e7: LDA  $40          ; a5 40
c7e9: CMP  #$27         ; c9 27
c7eb: BNE  $c7f1        ; d0 04
c7ed: LDA  #$01         ; a9 01
c7ef: BRA  $c84f        ; 80 5e
c7f1: LDA  $41          ; a5 41
c7f3: CMP  #$ed         ; c9 ed
c7f5: BNE  $c7fb        ; d0 04
c7f7: LDA  $40          ; a5 40
c7f9: CMP  #$28         ; c9 28
c7fb: BNE  $c801        ; d0 04
c7fd: LDA  #$00         ; a9 00
c7ff: BRA  $c84f        ; 80 4e
c801: LDA  $41          ; a5 41
c803: CMP  #$ed         ; c9 ed
c805: BNE  $c80b        ; d0 04
c807: LDA  $40          ; a5 40
c809: CMP  #$16         ; c9 16
c80b: BNE  $c811        ; d0 04
c80d: LDA  #$08         ; a9 08
c80f: BRA  $c84f        ; 80 3e
c811: LDA  $41          ; a5 41
c813: CMP  #$ed         ; c9 ed
c815: BNE  $c81b        ; d0 04
c817: LDA  $40          ; a5 40
c819: CMP  #$a2         ; c9 a2
c81b: BNE  $c821        ; d0 04
c81d: LDA  #$01         ; a9 01
c81f: BRA  $c84f        ; 80 2e
c821: LDA  $41          ; a5 41
c823: CMP  #$ed         ; c9 ed
c825: BNE  $c82b        ; d0 04
c827: LDA  $40          ; a5 40
c829: CMP  #$d9         ; c9 d9
c82b: BNE  $c831        ; d0 04
c82d: LDA  #$01         ; a9 01
c82f: BRA  $c84f        ; 80 1e
c831: LDA  $41          ; a5 41
c833: CMP  #$ed         ; c9 ed
c835: BNE  $c83b        ; d0 04
c837: LDA  $40          ; a5 40
c839: CMP  #$b9         ; c9 b9
c83b: BNE  $c841        ; d0 04
c83d: LDA  #$60         ; a9 60
c83f: BRA  $c84f        ; 80 0e
c841: LDA  $41          ; a5 41
c843: CMP  #$ed         ; c9 ed
c845: BNE  $c84b        ; d0 04
c847: LDA  $40          ; a5 40
c849: CMP  #$b9         ; c9 b9
c84b: BNE  $c851        ; d0 04
c84d: LDA  #$90         ; a9 90
c84f: SEC               ; 38 
c850: RTS               ; 60 
c851: CLC               ; 18 
c852: RTS               ; 60 
c853: .byte $0f   ;?
c854: STA  ($30)        ; 92 30
c856: LDA  $41          ; a5 41
c858: CMP  #$ed         ; c9 ed
c85a: BNE  $c860        ; d0 04
c85c: LDA  $40          ; a5 40
c85e: CMP  #$3e         ; c9 3e
c860: BEQ  $c880        ; f0 1e
c862: LDA  $41          ; a5 41
c864: CMP  #$ed         ; c9 ed
c866: BNE  $c86c        ; d0 04
c868: LDA  $40          ; a5 40
c86a: CMP  #$3f         ; c9 3f
c86c: BEQ  $c87a        ; f0 0c
c86e: LDA  $41          ; a5 41
c870: CMP  #$ed         ; c9 ed
c872: BNE  $c878        ; d0 04
c874: LDA  $40          ; a5 40
c876: CMP  #$40         ; c9 40
c878: BNE  $c886        ; d0 0c
c87a: LDA  ($40,X)      ; a1 40
c87c: AND  #$fd         ; 29 fd
c87e: BRA  $c884        ; 80 04
c880: LDA  ($40,X)      ; a1 40
c882: ORA  #$02         ; 09 02
c884: SEC               ; 38 
c885: RTS               ; 60 
c886: .byte $2f   ;?
c887: STA  ($24)        ; 92 24
c889: LDA  $41          ; a5 41
c88b: CMP  #$ed         ; c9 ed
c88d: BNE  $c893        ; d0 04
c88f: LDA  $40          ; a5 40
c891: CMP  #$01         ; c9 01
c893: BEQ  $c880        ; f0 eb
c895: LDA  $41          ; a5 41
c897: CMP  #$ed         ; c9 ed
c899: BNE  $c89f        ; d0 04
c89b: LDA  $40          ; a5 40
c89d: CMP  #$02         ; c9 02
c89f: BEQ  $c87a        ; f0 d9
c8a1: LDA  $41          ; a5 41
c8a3: CMP  #$ed         ; c9 ed
c8a5: BNE  $c8ab        ; d0 04
c8a7: LDA  $40          ; a5 40
c8a9: CMP  #$03         ; c9 03
c8ab: BEQ  $c87a        ; f0 cd
c8ad: .byte $1f   ;?
c8ae: STA  ($24)        ; 92 24
c8b0: LDA  $41          ; a5 41
c8b2: CMP  #$ed         ; c9 ed
c8b4: BNE  $c8ba        ; d0 04
c8b6: LDA  $40          ; a5 40
c8b8: CMP  #$1c         ; c9 1c
c8ba: BEQ  $c880        ; f0 c4
c8bc: LDA  $41          ; a5 41
c8be: CMP  #$ed         ; c9 ed
c8c0: BNE  $c8c6        ; d0 04
c8c2: LDA  $40          ; a5 40
c8c4: CMP  #$1d         ; c9 1d
c8c6: BEQ  $c87a        ; f0 b2
c8c8: LDA  $41          ; a5 41
c8ca: CMP  #$ed         ; c9 ed
c8cc: BNE  $c8d2        ; d0 04
c8ce: LDA  $40          ; a5 40
c8d0: CMP  #$1e         ; c9 1e
c8d2: BEQ  $c87a        ; f0 a6
c8d4: .byte $3f   ;?
c8d5: STA  ($27)        ; 92 27
c8d7: LDA  $41          ; a5 41
c8d9: CMP  #$ed         ; c9 ed
c8db: BNE  $c8e1        ; d0 04
c8dd: LDA  $40          ; a5 40
c8df: CMP  #$73         ; c9 73
c8e1: BEQ  $c880        ; f0 9d
c8e3: LDA  $41          ; a5 41
c8e5: CMP  #$ed         ; c9 ed
c8e7: BNE  $c8ed        ; d0 04
c8e9: LDA  $40          ; a5 40
c8eb: CMP  #$74         ; c9 74
c8ed: BEQ  $c87a        ; f0 8b
c8ef: LDA  $41          ; a5 41
c8f1: CMP  #$ed         ; c9 ed
c8f3: BNE  $c8f9        ; d0 04
c8f5: LDA  $40          ; a5 40
c8f7: CMP  #$75         ; c9 75
c8f9: BNE  $c8fe        ; d0 03
c8fb: JMP  ($c87a)      ; 4c 7a c8
c8fe: CLC               ; 18 
c8ff: RTS               ; 60 
c900: .byte $3f   ;?
c901: .byte $9b   ;?
c902: BIT  ($00a2)      ; 2c a2 00
c905: LDA  ($2c)        ; b2 2c
c907: RTI               ; 40 
c908: LDA  ($00)        ; b2 00
c90a: EOR  ($da,X)      ; 41 da
c90c: JSR  $eb9e        ; 20 9e eb
c90f: STA  $42          ; 85 42
c911: INX               ; e8 
c912: JSR  $eb9e        ; 20 9e eb
c915: STA  $43          ; 85 43
c917: ORA  $42          ; 05 42
c919: BNE  $c920        ; d0 05
c91b: PLX               ; fa 
c91c: INX               ; e8 
c91d: INX               ; e8 
c91e: BRA  $c92b        ; 80 0b
c920: PLX               ; fa 
c921: LDA  $42          ; a5 42
c923: STA  ($44),Y      ; 91 44
c925: INX               ; e8 
c926: LDA  $43          ; a5 43
c928: STA  ($44),Y      ; 91 44
c92a: INX               ; e8 
c92b: CPX  #$05         ; e0 05
c92d: BCC  $c90b        ; 90 dc
c92f: RTS               ; 60 
c930: PHX               ; da 
c931: LDX  #$00         ; a2 00
c933: LDA  $1492,X      ; bd 92 14
c936: BNE  $c93b        ; d0 03
c938: PLX               ; fa 
c939: CLC               ; 18 
c93a: RTS               ; 60 
c93b: LDA  $40          ; a5 40
c93d: CMP  $1492,X      ; dd 92 14
c940: BNE  $c951        ; d0 0f
c942: INX               ; e8 
c943: LDA  $41          ; a5 41
c945: CMP  $1492,X      ; dd 92 14
c948: BNE  $c952        ; d0 08
c94a: INX               ; e8 
c94b: LDA  $1492,X      ; bd 92 14
c94e: PLX               ; fa 
c94f: SEC               ; 38 
c950: RTS               ; 60 
c951: INX               ; e8 
c952: INX               ; e8 
c953: INX               ; e8 
c954: BRA  $c933        ; 80 dd
c956: CMP  ($04)        ; d2 04
c958: ASL  $8f0e        ; 0e 0e 8f
c95b: LDA  ($04,X)      ; a1 04
c95d: .byte $c2   ;?
c95e: TSB  $0e          ; 04 0e
c960: ASL  $01d2        ; 0e d2 01
c963: ASL  $600e        ; 0e 0e 60
c966: CMP  ($04)        ; d2 04
c968: ASL  $a90e        ; 0e 0e a9
c96b: BRK               ; 00 
c96c: STA  $0e0c,X      ; 8d 0c 0e
c96f: CMP  ($01)        ; d2 01
c971: ASL  $600e        ; 0e 0e 60
c974: SBC  ($0e)        ; f2 0e
c976: ASL  $1001        ; 0e 01 10
c979: CMP  ($04)        ; d2 04
c97b: ASL  $8f0e        ; 0e 0e 8f
c97e: LDA  ($04,X)      ; a1 04
c980: .byte $c2   ;?
c981: TSB  $0e          ; 04 0e
c983: ASL  $6a20        ; 0e 20 6a
c986: CMP  #$37         ; c9 37
c988: .byte $9f   ;?
c989: RTS               ; 60 
c98a: .byte $0f   ;?
c98b: LDY  #$02         ; a0 02
c98d: .byte $07   ;?
c98e: LDY  #$5f         ; a0 5f
c990: LDX  #$06         ; a2 06
c992: JSR  $1450        ; 20 50 14
c995: JSR  $1250        ; 20 50 12
c998: .byte $bf   ;?
c999: LDX  #$0a         ; a2 0a
c99b: .byte $57   ;?
c99c: LDX  #$37         ; a2 37
c99e: .byte $9f   ;?
c99f: .byte $2f   ;?
c9a0: .byte $9f   ;?
c9a1: .byte $03   ;?
c9a2: JSR  $c966        ; 20 66 c9
c9a5: CMP  ($01)        ; d2 01
c9a7: ASL  $0e,X        ; 06 0e
c9a9: RTS               ; 60 
c9aa: LDA  #$00         ; a9 00
c9ac: STA  $41          ; 85 41
c9ae: LDY  #$09         ; a0 09
c9b0: CLC               ; 18 
c9b1: ROR  $41,X        ; 66 41
c9b3: ROR  $40,X        ; 66 40
c9b5: BCC  $c9be        ; 90 07
c9b7: CLC               ; 18 
c9b8: LDA  $41          ; a5 41
c9ba: ADC  $44          ; 65 44
c9bc: STA  $41          ; 85 41
c9be: DEY               ; 88 
c9bf: BNE  $c9b1        ; d0 f0
c9c1: RTS               ; 60 
c9c2: PHP               ; 08 
c9c3: .byte $78   ;?
c9c4: LDA  $138b,X      ; ad 8b 13
c9c7: STA  $0e4c,X      ; 8d 4c 0e
c9ca: LDA  #$00         ; a9 00
c9cc: STA  $138b,X      ; 8d 8b 13
c9cf: CMP  ($04)        ; d2 04
c9d1: LSR  $ff0e        ; 4e 0e ff
c9d4: .byte $a3   ;?
c9d5: TSB  $c2          ; 04 c2
c9d7: TSB  $4e          ; 04 4e
c9d9: ASL  $a3f7        ; 0e f7 a3
c9dc: .byte $e2   ;?
c9dd: LSR  $040e        ; 4e 0e 04
c9e0: .byte $02   ;?
c9e1: .byte $77   ;?
c9e2: .byte $a3   ;?
c9e3: CMP  ($01)        ; d2 01
c9e5: LSR  $280e        ; 4e 0e 28
c9e8: RTS               ; 60 
c9e9: PHX               ; da 
c9ea: LDX  $138b        ; ae 8b 13
c9ed: CPX  #$21         ; e0 21
c9ef: BEQ  $c9fd        ; f0 0c
c9f1: LDX  $138b        ; ae 8b 13
c9f4: STA  $138c,X      ; 9d 8c 13
c9f7: INC  $138b        ; ee 8b 13
c9fa: CLC               ; 18 
c9fb: BRA  $c9fe        ; 80 01
c9fd: SEC               ; 38 
c9fe: PLX               ; fa 
c9ff: RTS               ; 60 
ca00: JSR  $1438        ; 20 38 14
ca03: .byte $07   ;?
ca04: .byte $9f   ;?
ca05: .byte $17   ;?
ca06: .byte $9f   ;?
ca07: .byte $27   ;?
ca08: .byte $9f   ;?
ca09: LDX  #$06         ; a2 06
ca0b: LDA  ($40),Y      ; b1 40
ca0d: STA  $46          ; 85 46
ca0f: INX               ; e8 
ca10: LDA  ($40),Y      ; b1 40
ca12: STA  $47          ; 85 47
ca14: LDA  ($40,X)      ; a1 40
ca16: AND  #$60         ; 29 60
ca18: LSR               ; 4a 
ca19: LSR               ; 4a 
ca1a: LSR               ; 4a 
ca1b: LSR               ; 4a 
ca1c: TAX               ; aa 
ca1d: CPX  #$08         ; e0 08
ca1f: BCS  $ca24        ; b0 03
ca21: JMP  ($ec89,X)    ; 7c 89 ec
ca24: .byte $87   ;?
ca25: .byte $9f   ;?
ca26: RTS               ; 60 
ca27: JSR  $144d        ; 20 4d 14
ca2a: LDX  #$01         ; a2 01
ca2c: LDA  ($40),Y      ; b1 40
ca2e: BEQ  $ca3b        ; f0 0b
ca30: CMP  #$01         ; c9 01
ca32: BEQ  $ca3b        ; f0 07
ca34: CMP  #$03         ; c9 03
ca36: BEQ  $ca3b        ; f0 03
ca38: .byte $af   ;?
ca39: LDY  #$09         ; a0 09
ca3b: ASL               ; 0a 
ca3c: TAX               ; aa 
ca3d: CPX  #$1a         ; e0 1a
ca3f: BCS  $ca44        ; b0 03
ca41: JMP  ($ec91,X)    ; 7c 91 ec
ca44: .byte $87   ;?
ca45: .byte $9f   ;?
ca46: RTS               ; 60 
ca47: LDX  #$02         ; a2 02
ca49: LDA  ($40),Y      ; b1 40
ca4b: BNE  $ca7e        ; d0 31
ca4d: INX               ; e8 
ca4e: LDA  ($40),Y      ; b1 40
ca50: BNE  $ca7e        ; d0 2c
ca52: LDX  #$06         ; a2 06
ca54: LDA  ($40),Y      ; b1 40
ca56: CMP  #$02         ; c9 02
ca58: BNE  $ca7e        ; d0 24
ca5a: INX               ; e8 
ca5b: LDA  ($40),Y      ; b1 40
ca5d: BNE  $ca7e        ; d0 1f
ca5f: LDA  $137d,X      ; ad 7d 13
ca62: CMP  #$03         ; c9 03
ca64: BEQ  $ca7e        ; f0 18
ca66: CMP  #$04         ; c9 04
ca68: BNE  $ca81        ; d0 17
ca6a: LDA  ($40,X)      ; a1 40
ca6c: AND  #$1f         ; 29 1f
ca6e: CMP  #$00         ; c9 00
ca70: BEQ  $ca85        ; f0 13
ca72: CMP  #$02         ; c9 02
ca74: BNE  $ca7e        ; d0 08
ca76: LDX  #$04         ; a2 04
ca78: LDA  ($40),Y      ; b1 40
ca7a: AND  #$0f         ; 29 0f
ca7c: BEQ  $ca85        ; f0 07
ca7e: .byte $87   ;?
ca7f: .byte $9f   ;?
ca80: RTS               ; 60 
ca81: CMP  #$05         ; c9 05
ca83: BNE  $ca7e        ; d0 f9
ca85: LDA  #$00         ; a9 00
ca87: STA  $1388,X      ; 8d 88 13
ca8a: STA  $1389,X      ; 8d 89 13
ca8d: LDA  ($40,X)      ; a1 40
ca8f: AND  #$1f         ; 29 1f
ca91: CMP  #$03         ; c9 03
ca93: BCS  $ca7e        ; b0 e9
ca95: ASL               ; 0a 
ca96: TAX               ; aa 
ca97: JMP  ($ecab,X)    ; 7c ab ec
ca9a: CLC               ; 18 
ca9b: .byte $4f   ;?
ca9c: LDA  ($01,X)      ; a1 01
ca9e: SEC               ; 38 
ca9f: ROL  $1388        ; 2e 88 13
caa2: CLC               ; 18 
caa3: .byte $df   ;?
caa4: .byte $4f   ;?
caa5: ORA  ($38,X)      ; 01 38
caa7: ROL  $1388        ; 2e 88 13
caaa: BRA  $caef        ; 80 43
caac: BRA  $caef        ; 80 41
caae: LDX  #$04         ; a2 04
cab0: LDA  ($40),Y      ; b1 40
cab2: AND  #$0f         ; 29 0f
cab4: BEQ  $cae7        ; f0 31
cab6: CMP  #$01         ; c9 01
cab8: BEQ  $cae0        ; f0 26
caba: CMP  #$02         ; c9 02
cabc: BEQ  $cacd        ; f0 0f
cabe: CMP  #$03         ; c9 03
cac0: BEQ  $ca7e        ; f0 bc
cac2: CMP  #$04         ; c9 04
cac4: BNE  $ca7e        ; d0 b8
cac6: CLC               ; 18 
cac7: .byte $7f   ;?
cac8: LDX  #$22         ; a2 22
caca: SEC               ; 38 
cacb: BRA  $caec        ; 80 1f
cacd: LDA  ($40),Y      ; b1 40
cacf: ASL               ; 0a 
cad0: BCC  $cad9        ; 90 07
cad2: CLC               ; 18 
cad3: .byte $4f   ;?
cad4: LDY  #$16         ; a0 16
cad6: SEC               ; 38 
cad7: BRA  $caec        ; 80 13
cad9: CLC               ; 18 
cada: .byte $5f   ;?
cadb: LDY  #$0f         ; a0 0f
cadd: SEC               ; 38 
cade: BRA  $caec        ; 80 0c
cae0: CLC               ; 18 
cae1: .byte $3f   ;?
cae2: LDY  #$08         ; a0 08
cae4: SEC               ; 38 
cae5: BRA  $caec        ; 80 05
cae7: CLC               ; 18 
cae8: .byte $2f   ;?
cae9: LDY  #$01         ; a0 01
caeb: SEC               ; 38 
caec: ROL  $1388        ; 2e 88 13
caef: LDA  ($88)        ; b2 88
caf1: RTI               ; 40 
caf2: LDA  ($13)        ; b2 13
caf4: EOR  ($b2,X)      ; 41 b2
caf6: .byte $02   ;?
caf7: TSB  $b2          ; 44 b2
caf9: BRK               ; 00 
cafa: EOR  $60          ; 45 60
cafc: LDA  $137d,X      ; ad 7d 13
caff: CMP  #$03         ; c9 03
cb01: BEQ  $cb39        ; f0 36
cb03: CMP  #$04         ; c9 04
cb05: BNE  $cb19        ; d0 12
cb07: LDA  ($40,X)      ; a1 40
cb09: AND  #$1f         ; 29 1f
cb0b: CMP  #$02         ; c9 02
cb0d: BNE  $cb39        ; d0 2a
cb0f: LDX  #$04         ; a2 04
cb11: LDA  ($40),Y      ; b1 40
cb13: AND  #$0f         ; 29 0f
cb15: BEQ  $cb1d        ; f0 06
cb17: BRA  $cb39        ; 80 20
cb19: CMP  #$05         ; c9 05
cb1b: BNE  $cb39        ; d0 1c
cb1d: LDA  ($40,X)      ; a1 40
cb1f: AND  #$1f         ; 29 1f
cb21: CMP  #$00         ; c9 00
cb23: BNE  $cb3c        ; d0 17
cb25: LDX  #$02         ; a2 02
cb27: LDA  ($40),Y      ; b1 40
cb29: CMP  #$01         ; c9 01
cb2b: BNE  $cb31        ; d0 04
cb2d: .byte $47   ;?
cb2e: LDA  ($80,X)      ; a1 80
cb30: LSR  $c9,X        ; 46 c9
cb32: BRK               ; 00 
cb33: BEQ  $cb77        ; f0 42
cb35: CMP  #$02         ; c9 02
cb37: BNE  $cb39        ; d0 00
cb39: .byte $87   ;?
cb3a: .byte $9f   ;?
cb3b: RTS               ; 60 
cb3c: CMP  #$02         ; c9 02
cb3e: BNE  $cb39        ; d0 f9
cb40: LDX  #$02         ; a2 02
cb42: LDA  ($40),Y      ; b1 40
cb44: CMP  #$00         ; c9 00
cb46: BNE  $cb39        ; d0 f1
cb48: LDX  #$04         ; a2 04
cb4a: LDA  ($40),Y      ; b1 40
cb4c: AND  #$0f         ; 29 0f
cb4e: BEQ  $cb75        ; f0 25
cb50: CMP  #$01         ; c9 01
cb52: BEQ  $cb71        ; f0 1d
cb54: CMP  #$02         ; c9 02
cb56: BEQ  $cb64        ; f0 0c
cb58: CMP  #$03         ; c9 03
cb5a: BEQ  $cb39        ; f0 dd
cb5c: CMP  #$04         ; c9 04
cb5e: BNE  $cb39        ; d0 d9
cb60: .byte $77   ;?
cb61: LDX  #$80         ; a2 80
cb63: .byte $13   ;?
cb64: LDA  ($40),Y      ; b1 40
cb66: ASL               ; 0a 
cb67: BCC  $cb6d        ; 90 04
cb69: .byte $47   ;?
cb6a: LDY  #$80         ; a0 80
cb6c: ASL               ; 0a 
cb6d: .byte $57   ;?
cb6e: LDY  #$80         ; a0 80
cb70: ASL  $37,X        ; 06 37
cb72: LDY  #$80         ; a0 80
cb74: .byte $02   ;?
cb75: .byte $27   ;?
cb76: LDY  #$a7         ; a0 a7
cb78: .byte $9f   ;?
cb79: RTS               ; 60 
cb7a: LDA  $137d,X      ; ad 7d 13
cb7d: CMP  #$03         ; c9 03
cb7f: BNE  $cb8b        ; d0 0a
cb81: LDA  ($40,X)      ; a1 40
cb83: AND  #$1f         ; 29 1f
cb85: CMP  #$00         ; c9 00
cb87: BNE  $cb9f        ; d0 16
cb89: BRA  $cbc3        ; 80 38
cb8b: CMP  #$04         ; c9 04
cb8d: BNE  $cba2        ; d0 13
cb8f: LDA  ($40,X)      ; a1 40
cb91: AND  #$1f         ; 29 1f
cb93: CMP  #$02         ; c9 02
cb95: BNE  $cb9f        ; d0 08
cb97: LDX  #$04         ; a2 04
cb99: LDA  ($40),Y      ; b1 40
cb9b: AND  #$0f         ; 29 0f
cb9d: BEQ  $cba6        ; f0 07
cb9f: .byte $87   ;?
cba0: .byte $9f   ;?
cba1: RTS               ; 60 
cba2: CMP  #$05         ; c9 05
cba4: BNE  $cb9f        ; d0 f9
cba6: LDA  ($40,X)      ; a1 40
cba8: AND  #$1f         ; 29 1f
cbaa: CMP  #$00         ; c9 00
cbac: BNE  $cbd0        ; d0 22
cbae: LDX  #$02         ; a2 02
cbb0: LDA  ($40),Y      ; b1 40
cbb2: CMP  #$01         ; c9 01
cbb4: BNE  $cbbb        ; d0 05
cbb6: .byte $c7   ;?
cbb7: LDA  ($4c,X)      ; a1 4c
cbb9: .byte $2f   ;?
cbba: CPY  ($00c9)      ; cc c9 00
cbbd: BEQ  $cbb8        ; f0 f9
cbbf: CMP  #$02         ; c9 02
cbc1: BNE  $cb9f        ; d0 dc
cbc3: LDX  #$04         ; a2 04
cbc5: LDA  ($40),Y      ; b1 40
cbc7: BNE  $cb9f        ; d0 d6
cbc9: LDX  #$05         ; a2 05
cbcb: LDA  ($40),Y      ; b1 40
cbcd: JMP  ($cc2f)      ; 4c 2f cc
cbd0: CMP  #$02         ; c9 02
cbd2: BNE  $cb9f        ; d0 cb
cbd4: LDX  #$02         ; a2 02
cbd6: LDA  ($40),Y      ; b1 40
cbd8: CMP  #$00         ; c9 00
cbda: BNE  $cb9f        ; d0 c3
cbdc: LDX  #$04         ; a2 04
cbde: LDA  ($40),Y      ; b1 40
cbe0: AND  #$0f         ; 29 0f
cbe2: BEQ  $cc29        ; f0 45
cbe4: CMP  #$01         ; c9 01
cbe6: BEQ  $cc1d        ; f0 35
cbe8: CMP  #$02         ; c9 02
cbea: BEQ  $cc00        ; f0 14
cbec: CMP  #$03         ; c9 03
cbee: BEQ  $cb9f        ; f0 af
cbf0: CMP  #$04         ; c9 04
cbf2: BNE  $cb9f        ; d0 ab
cbf4: .byte $c2   ;?
cbf5: BPL  $cc45        ; 10 4e
cbf7: ASL  $a2f7        ; 0e f7 a2
cbfa: CMP  ($02)        ; d2 02
cbfc: LSR  $800e        ; 4e 0e 80
cbff: .byte $2f   ;?
cc00: LDA  ($40),Y      ; b1 40
cc02: ASL               ; 0a 
cc03: BCC  $cc11        ; 90 0c
cc05: .byte $c2   ;?
cc06: BPL  $cc36        ; 10 2e
cc08: ASL  $a0c7        ; 0e c7 a0
cc0b: CMP  ($02)        ; d2 02
cc0d: ROL  $800e        ; 2e 0e 80
cc10: ASL  $10c2        ; 1e c2 10
cc13: ROL  $0e,X        ; 26 0e
cc15: .byte $d7   ;?
cc16: LDY  #$d2         ; a0 d2
cc18: .byte $02   ;?
cc19: ROL  $0e,X        ; 26 0e
cc1b: BRA  $cc2f        ; 80 12
cc1d: .byte $c2   ;?
cc1e: BPL  $cc3e        ; 10 1e
cc20: ASL  $a0b7        ; 0e b7 a0
cc23: CMP  ($02)        ; d2 02
cc25: ASL  $800e        ; 1e 0e 80
cc28: ASL  $c2,X        ; 06 c2
cc2a: BPL  $cc32        ; 10 06
cc2c: ASL  $a0a7        ; 0e a7 a0
cc2f: .byte $a7   ;?
cc30: .byte $9f   ;?
cc31: RTS               ; 60 
cc32: LDA  $137d,X      ; ad 7d 13
cc35: CMP  #$03         ; c9 03
cc37: BEQ  $cc3d        ; f0 04
cc39: CMP  #$04         ; c9 04
cc3b: BNE  $cc70        ; d0 33
cc3d: LDX  #$02         ; a2 02
cc3f: LDA  ($40),Y      ; b1 40
cc41: CMP  #$80         ; c9 80
cc43: BCS  $cc70        ; b0 2b
cc45: STA  $137f,X      ; 8d 7f 13
cc48: BNE  $cc5a        ; d0 10
cc4a: .byte $27   ;?
cc4b: .byte $93   ;?
cc4c: LDA  #$35         ; a9 35
cc4e: STA  $124b,X      ; 8d 4b 12
cc51: LDA  #$bd         ; a9 bd
cc53: STA  $124c,X      ; 8d 4c 12
cc56: .byte $a7   ;?
cc57: .byte $93   ;?
cc58: BRA  $cc6d        ; 80 13
cc5a: LDA  #$04         ; a9 04
cc5c: STA  $137d,X      ; 8d 7d 13
cc5f: .byte $27   ;?
cc60: .byte $93   ;?
cc61: LDA  #$56         ; a9 56
cc63: STA  $124b,X      ; 8d 4b 12
cc66: LDA  #$bd         ; a9 bd
cc68: STA  $124c,X      ; 8d 4c 12
cc6b: .byte $a7   ;?
cc6c: .byte $93   ;?
cc6d: .byte $a7   ;?
cc6e: .byte $9f   ;?
cc6f: RTS               ; 60 
cc70: .byte $87   ;?
cc71: .byte $9f   ;?
cc72: RTS               ; 60 
cc73: LDA  ($40,X)      ; a1 40
cc75: AND  #$01         ; 29 01
cc77: BNE  $cc92        ; d0 19
cc79: LDX  #$02         ; a2 02
cc7b: LDA  ($40),Y      ; b1 40
cc7d: STA  $45          ; 85 45
cc7f: LDX  #$03         ; a2 03
cc81: LDA  ($40),Y      ; b1 40
cc83: TAX               ; aa 
cc84: DEX               ; ca 
cc85: TXA               ; 8a 
cc86: ASL               ; 0a 
cc87: TAX               ; aa 
cc88: CPX  #$10         ; e0 10
cc8a: BCS  $cc8f        ; b0 03
cc8c: JMP  ($ecb1,X)    ; 7c b1 ec
cc8f: .byte $87   ;?
cc90: .byte $9f   ;?
cc91: RTS               ; 60 
cc92: LDX  #$03         ; a2 03
cc94: LDA  ($40),Y      ; b1 40
cc96: CMP  #$21         ; c9 21
cc98: BNE  $cc9d        ; d0 03
cc9a: JMP  ($cee6)      ; 4c e6 ce
cc9d: CMP  #$22         ; c9 22
cc9f: BNE  $cc8f        ; d0 ee
cca1: JMP  ($cecd)      ; 4c cd ce
cca4: LDA  ($c1)        ; b2 c1
cca6: RTI               ; 40 
cca7: LDA  ($ec)        ; b2 ec
cca9: EOR  ($b2,X)      ; 41 b2
ccab: ORA  ($44)        ; 12 44
ccad: JSR  $cccc        ; 20 cc cc
ccb0: LDA  $1005,X      ; ad 05 10
ccb3: STA  $1347,X      ; 8d 47 13
ccb6: LDA  ($45)        ; b2 45
ccb8: TSB  $b2          ; 44 b2
ccba: .byte $13   ;?
ccbb: EOR  $20          ; 45 20
ccbd: BRK               ; 00 
ccbe: CMP  #$b2         ; c9 b2
ccc0: AND  $b240,X      ; 3d 40 b2
ccc3: .byte $13   ;?
ccc4: EOR  ($b2,X)      ; 41 b2
ccc6: ORA  ($44)        ; 12 44
ccc8: LDA  ($00)        ; b2 00
ccca: EOR  $60          ; 45 60
cccc: LDX  #$00         ; a2 00
ccce: LDA  ($40),Y      ; b1 40
ccd0: STA  $133d,X      ; 9d 3d 13
ccd3: INX               ; e8 
ccd4: CPX  $44          ; e4 44
ccd6: BCC  $ccce        ; 90 f6
ccd8: RTS               ; 60 
ccd9: JSR  $1441        ; 20 41 14
ccdc: LDA  ($d3)        ; b2 d3
ccde: RTI               ; 40 
ccdf: LDA  ($ec)        ; b2 ec
cce1: EOR  ($b2,X)      ; 41 b2
cce3: .byte $c2   ;?
cce4: TSB  $b2          ; 44 b2
cce6: ORA  ($45,X)      ; 01 45
cce8: .byte $2f   ;?
cce9: .byte $4f   ;?
ccea: PHP               ; 08 
cceb: LDA  ($e8)        ; b2 e8
cced: TSB  $b2          ; 44 b2
ccef: BRK               ; 00 
ccf0: EOR  $80          ; 45 80
ccf2: ORA  $4f3f,X      ; 1d 3f 4f
ccf5: ORA  ($cf),Y      ; 11 cf
ccf7: .byte $4f   ;?
ccf8: .byte $07   ;?
ccf9: LDA  ($0a)        ; b2 0a
ccfb: TSB  $b2          ; 44 b2
ccfd: ORA  ($45,X)      ; 01 45
ccff: RTS               ; 60 
cd00: LDA  ($2d)        ; b2 2d
cd02: TSB  $b2          ; 44 b2
cd04: ORA  ($45,X)      ; 01 45
cd06: RTS               ; 60 
cd07: .byte $cf   ;?
cd08: .byte $4f   ;?
cd09: ASL  $b2,X        ; 06 b2
cd0b: .byte $ab   ;?
cd0c: TSB  $b2          ; 44 b2
cd0e: ORA  ($45,X)      ; 01 45
cd10: .byte $1f   ;?
cd11: EOR  $3807,X      ; 4d 07 38
cd14: LDA  $44          ; a5 44
cd16: SBC  #$04         ; e9 04
cd18: STA  $44          ; 85 44
cd1a: RTS               ; 60 
cd1b: LDA  $45          ; a5 45
cd1d: CMP  #$09         ; c9 09
cd1f: BCC  $cd2f        ; 90 0e
cd21: CMP  #$21         ; c9 21
cd23: BEQ  $cd71        ; f0 4c
cd25: CMP  #$29         ; c9 29
cd27: BNE  $cd2c        ; d0 03
cd29: JMP  ($cd95)      ; 4c 95 cd
cd2c: .byte $87   ;?
cd2d: .byte $9f   ;?
cd2e: RTS               ; 60 
cd2f: ASL               ; 0a 
cd30: TAX               ; aa 
cd31: PHX               ; da 
cd32: .byte $3f   ;?
cd33: .byte $9b   ;?
cd34: AND  #$b2         ; 29 b2
cd36: AND  ($40)        ; 32 40
cd38: LDA  ($00)        ; b2 00
cd3a: EOR  ($20,X)      ; 41 20
cd3c: STZ  $85eb,X      ; 9e eb 85
cd3f: .byte $42   ;?
cd40: INX               ; e8 
cd41: JSR  $eb9e        ; 20 9e eb
cd44: STA  $43          ; 85 43
cd46: CMP  #$ff         ; c9 ff
cd48: BEQ  $cd5e        ; f0 14
cd4a: ORA  $42          ; 05 42
cd4c: BEQ  $cd5e        ; f0 10
cd4e: PLX               ; fa 
cd4f: LDA  $42          ; a5 42
cd51: STA  $40          ; 85 40
cd53: LDA  $43          ; a5 43
cd55: STA  $41          ; 85 41
cd57: .byte $e7   ;?
cd58: TYA               ; 98 
cd59: JSR  $eb95        ; 20 95 eb
cd5c: BRA  $cd6b        ; 80 0d
cd5e: PLX               ; fa 
cd5f: LDA  $f10b,X      ; bd 0b f1
cd62: STA  $40          ; 85 40
cd64: LDA  $f10c,X      ; bd 0c f1
cd67: STA  $41          ; 85 41
cd69: LDA  ($40,X)      ; a1 40
cd6b: STA  $44          ; 85 44
cd6d: LDA  ($00)        ; b2 00
cd6f: EOR  $60          ; 45 60
cd71: .byte $3f   ;?
cd72: .byte $9b   ;?
cd73: .byte $19   ;?
cd74: LDA  ($40)        ; b2 40
cd76: RTI               ; 40 
cd77: LDA  ($00)        ; b2 00
cd79: EOR  ($20,X)      ; 41 20
cd7b: STA  $eb          ; 95 eb
cd7d: STA  $42          ; 85 42
cd7f: INX               ; e8 
cd80: JSR  $eb9e        ; 20 9e eb
cd83: STA  $43          ; 85 43
cd85: CMP  #$ff         ; c9 ff
cd87: BEQ  $cd8d        ; f0 04
cd89: ORA  $42          ; 05 42
cd8b: BNE  $cd4f        ; d0 c2
cd8d: LDA  ($73)        ; b2 73
cd8f: RTI               ; 40 
cd90: LDA  ($f1)        ; b2 f1
cd92: EOR  ($80,X)      ; 41 80
cd94: CPY  $b2          ; d4 b2
cd96: ORA  $40          ; 05 40
cd98: LDA  ($f2)        ; b2 f2
cd9a: EOR  ($80,X)      ; 41 80
cd9c: CPY  ($8760)      ; cc 60 87
cd9f: .byte $9f   ;?
cda0: RTS               ; 60 
cda1: RTS               ; 60 
cda2: LDA  $137d,X      ; ad 7d 13
cda5: CMP  #$03         ; c9 03
cda7: BEQ  $cdc6        ; f0 1d
cda9: CMP  #$04         ; c9 04
cdab: BEQ  $cdb9        ; f0 0c
cdad: CMP  #$05         ; c9 05
cdaf: BNE  $cdc6        ; d0 15
cdb1: LDA  ($84)        ; b2 84
cdb3: RTI               ; 40 
cdb4: LDA  ($13)        ; b2 13
cdb6: EOR  ($80,X)      ; 41 80
cdb8: ASL  $b2,X        ; 06 b2
cdba: .byte $2f   ;?
cdbb: RTI               ; 40 
cdbc: LDA  ($f2)        ; b2 f2
cdbe: EOR  ($b2,X)      ; 41 b2
cdc0: ORA  ($44,X)      ; 01 44
cdc2: LDA  ($00)        ; b2 00
cdc4: EOR  $60          ; 45 60
cdc6: RTS               ; 60 
cdc7: LDX  #$02         ; a2 02
cdc9: LDA  ($40),Y      ; b1 40
cdcb: BNE  $ce16        ; d0 49
cdcd: INX               ; e8 
cdce: LDA  ($40),Y      ; b1 40
cdd0: BNE  $ce16        ; d0 44
cdd2: LDX  #$06         ; a2 06
cdd4: LDA  ($40),Y      ; b1 40
cdd6: CMP  #$01         ; c9 01
cdd8: BNE  $ce16        ; d0 3c
cdda: INX               ; e8 
cddb: LDA  ($40),Y      ; b1 40
cddd: BNE  $ce16        ; d0 37
cddf: LDA  $137d,X      ; ad 7d 13
cde2: CMP  #$03         ; c9 03
cde4: BEQ  $ce16        ; f0 30
cde6: CMP  #$04         ; c9 04
cde8: BEQ  $ce14        ; f0 2a
cdea: CMP  #$05         ; c9 05
cdec: BNE  $ce16        ; d0 28
cdee: LDX  #$04         ; a2 04
cdf0: LDA  ($40),Y      ; b1 40
cdf2: CMP  #$05         ; c9 05
cdf4: BCS  $ce16        ; b0 20
cdf6: JSR  $ce17        ; 20 17 ce
cdf9: TAX               ; aa 
cdfa: LDA  ($85)        ; b2 85
cdfc: RTI               ; 40 
cdfd: LDA  ($13)        ; b2 13
cdff: EOR  ($e0,X)      ; 41 e0
ce01: BRK               ; 00 
ce02: BEQ  $ce0d        ; f0 09
ce04: INC  $40,X        ; e6 40
ce06: BNE  $ce0a        ; d0 02
ce08: INC  $41,X        ; e6 41
ce0a: DEX               ; ca 
ce0b: BRA  $ce02        ; 80 f5
ce0d: LDA  ($01)        ; b2 01
ce0f: TSB  $b2          ; 44 b2
ce11: BRK               ; 00 
ce12: EOR  $60          ; 45 60
ce14: .byte $87   ;?
ce15: .byte $9f   ;?
ce16: RTS               ; 60 
ce17: CMP  #$00         ; c9 00
ce19: BEQ  $ce2b        ; f0 10
ce1b: .byte $2f   ;?
ce1c: .byte $4f   ;?
ce1d: PHP               ; 08 
ce1e: CMP  #$02         ; c9 02
ce20: BNE  $ce2b        ; d0 09
ce22: LDA  #$03         ; a9 03
ce24: BRA  $ce2b        ; 80 05
ce26: .byte $3f   ;?
ce27: .byte $4f   ;?
ce28: .byte $02   ;?
ce29: BIT  #$01         ; 89 01
ce2b: RTS               ; 60 
ce2c: LDX  #$02         ; a2 02
ce2e: LDA  ($40),Y      ; b1 40
ce30: CMP  #$04         ; c9 04
ce32: BCS  $ce7c        ; b0 48
ce34: INX               ; e8 
ce35: LDA  ($40),Y      ; b1 40
ce37: BNE  $ce7c        ; d0 43
ce39: INX               ; e8 
ce3a: LDA  ($40),Y      ; b1 40
ce3c: CMP  #$04         ; c9 04
ce3e: BCS  $ce7c        ; b0 3c
ce40: INX               ; e8 
ce41: LDA  ($40),Y      ; b1 40
ce43: BNE  $ce7c        ; d0 37
ce45: INX               ; e8 
ce46: LDA  ($40),Y      ; b1 40
ce48: BNE  $ce7c        ; d0 32
ce4a: INX               ; e8 
ce4b: LDA  ($40),Y      ; b1 40
ce4d: BNE  $ce7c        ; d0 2d
ce4f: LDA  $137d,X      ; ad 7d 13
ce52: CMP  #$03         ; c9 03
ce54: BEQ  $ce7c        ; f0 26
ce56: CMP  #$04         ; c9 04
ce58: BEQ  $ce7a        ; f0 20
ce5a: CMP  #$05         ; c9 05
ce5c: BNE  $ce7c        ; d0 1e
ce5e: LDX  #$04         ; a2 04
ce60: LDA  ($40),Y      ; b1 40
ce62: JSR  $ce17        ; 20 17 ce
ce65: TAY               ; a8 
ce66: LDX  #$02         ; a2 02
ce68: LDA  ($40),Y      ; b1 40
ce6a: .byte $99   ;?
ce6b: STA  $13          ; 85 13
ce6d: CPY  #$01         ; c0 01
ce6f: BNE  $ce77        ; d0 06
ce71: CMP  #$00         ; c9 00
ce73: BNE  $ce77        ; d0 02
ce75: .byte $a7   ;?
ce76: LDX  #$a7         ; a2 a7
ce78: .byte $9f   ;?
ce79: RTS               ; 60 
ce7a: .byte $87   ;?
ce7b: .byte $9f   ;?
ce7c: RTS               ; 60 
ce7d: .byte $87   ;?
ce7e: .byte $9f   ;?
ce7f: RTS               ; 60 
ce80: .byte $87   ;?
ce81: .byte $9f   ;?
ce82: RTS               ; 60 
ce83: LDA  $137d,X      ; ad 7d 13
ce86: CMP  #$03         ; c9 03
ce88: BEQ  $ceca        ; f0 40
ce8a: CMP  #$04         ; c9 04
ce8c: BEQ  $ce92        ; f0 04
ce8e: CMP  #$05         ; c9 05
ce90: BNE  $ceca        ; d0 38
ce92: LDX  #$02         ; a2 02
ce94: LDA  ($40),Y      ; b1 40
ce96: BNE  $cea8        ; d0 10
ce98: .byte $27   ;?
ce99: .byte $93   ;?
ce9a: LDA  #$56         ; a9 56
ce9c: STA  $124b,X      ; 8d 4b 12
ce9f: LDA  #$bd         ; a9 bd
cea1: STA  $124c,X      ; 8d 4c 12
cea4: .byte $a7   ;?
cea5: .byte $93   ;?
cea6: BRA  $cec7        ; 80 1f
cea8: LDX  #$00         ; a2 00
ceaa: CMP  $f230,X      ; dd 30 f2
cead: BEQ  $ceb6        ; f0 07
ceaf: INX               ; e8 
ceb0: CPX  #$01         ; e0 01
ceb2: BCC  $ceaa        ; 90 f6
ceb4: BRA  $ceca        ; 80 14
ceb6: STA  $1384,X      ; 8d 84 13
ceb9: .byte $27   ;?
ceba: .byte $93   ;?
cebb: LDA  #$72         ; a9 72
cebd: STA  $124b,X      ; 8d 4b 12
cec0: LDA  #$bd         ; a9 bd
cec2: STA  $124c,X      ; 8d 4c 12
cec5: .byte $a7   ;?
cec6: .byte $93   ;?
cec7: .byte $a7   ;?
cec8: .byte $9f   ;?
cec9: RTS               ; 60 
ceca: .byte $87   ;?
cecb: .byte $9f   ;?
cecc: RTS               ; 60 
cecd: JSR  $1444        ; 20 44 14
ced0: LDA  ($d1)        ; b2 d1
ced2: RTI               ; 40 
ced3: LDA  ($ee)        ; b2 ee
ced5: EOR  ($b2,X)      ; 41 b2
ced7: .byte $78   ;?
ced8: TSB  $b2          ; 44 b2
ceda: BRK               ; 00 
cedb: EOR  $7f          ; 45 7f
cedd: .byte $4f   ;?
cede: ASL  $b2,X        ; 06 b2
cee0: DEA               ; 3a 
cee1: TSB  $b2          ; 44 b2
cee3: .byte $02   ;?
cee4: EOR  $60          ; 45 60
cee6: LDA  ($85)        ; b2 85
cee8: RTI               ; 40 
cee9: LDA  ($ee)        ; b2 ee
ceeb: EOR  ($b2,X)      ; 41 b2
ceed: ORA  #$44         ; 09 44
ceef: LDA  ($00)        ; b2 00
cef1: EOR  $60          ; 45 60
cef3: JSR  $144a        ; 20 4a 14
cef6: LDA  ($40,X)      ; a1 40
cef8: TAX               ; aa 
cef9: AND  #$1f         ; 29 1f
cefb: CMP  #$01         ; c9 01
cefd: BEQ  $cf11        ; f0 12
ceff: TXA               ; 8a 
cf00: AND  #$1f         ; 29 1f
cf02: CMP  #$02         ; c9 02
cf04: BEQ  $cf09        ; f0 03
cf06: JMP  ($cff2)      ; 4c f2 cf
cf09: LDA  ($1a)        ; b2 1a
cf0b: .byte $42   ;?
cf0c: LDA  ($f5)        ; b2 f5
cf0e: .byte $43   ;?
cf0f: BRA  $cf3e        ; 80 2d
cf11: LDX  #$04         ; a2 04
cf13: LDA  ($40),Y      ; b1 40
cf15: .byte $af   ;?
cf16: .byte $4f   ;?
cf17: ORA  $bf          ; 05 bf
cf19: .byte $4f   ;?
cf1a: .byte $02   ;?
cf1b: BRA  $cf25        ; 80 08
cf1d: CMP  #$02         ; c9 02
cf1f: BNE  $cf38        ; d0 17
cf21: LDA  #$03         ; a9 03
cf23: STA  ($40),Y      ; 91 40
cf25: CMP  #$03         ; c9 03
cf27: BCC  $cf38        ; 90 0f
cf29: CMP  #$03         ; c9 03
cf2b: BEQ  $cf30        ; f0 03
cf2d: JMP  ($cfa4)      ; 4c a4 cf
cf30: LDA  ($6b)        ; b2 6b
cf32: .byte $42   ;?
cf33: LDA  ($f5)        ; b2 f5
cf35: .byte $43   ;?
cf36: BRA  $cf3e        ; 80 06
cf38: LDA  ($31)        ; b2 31
cf3a: .byte $42   ;?
cf3b: LDA  ($f2)        ; b2 f2
cf3d: .byte $43   ;?
cf3e: LDA  ($42,X)      ; a1 42
cf40: CMP  #$ff         ; c9 ff
cf42: BEQ  $cf2d        ; f0 e9
cf44: LDY  #$08         ; a0 08
cf46: LDX  #$01         ; a2 01
cf48: LDA  ($42,X)      ; a1 42
cf4a: CMP  ($40),Y      ; d1 40
cf4c: BEQ  $cf59        ; f0 0b
cf4e: TYA               ; 98 
cf4f: STZ  $42          ; 64 42
cf51: STA  $42          ; 85 42
cf53: BCC  $cf57        ; 90 02
cf55: INC  $43,X        ; e6 43
cf57: BRA  $cf3e        ; 80 e5
cf59: DEY               ; 88 
cf5a: INC  $42,X        ; e6 42
cf5c: BNE  $cf60        ; d0 02
cf5e: INC  $43,X        ; e6 43
cf60: INX               ; e8 
cf61: CPX  #$06         ; e0 06
cf63: BNE  $cf48        ; d0 e3
cf65: LDA  ($42,X)      ; a1 42
cf67: BEQ  $cf94        ; f0 2b
cf69: CMP  #$ff         ; c9 ff
cf6b: BEQ  $cf7e        ; f0 11
cf6d: STA  $44          ; 85 44
cf6f: LDA  ($00)        ; b2 00
cf71: EOR  $a2          ; 45 a2
cf73: ORA  ($b1,X)      ; 01 b1
cf75: .byte $42   ;?
cf76: STA  $40          ; 85 40
cf78: INX               ; e8 
cf79: LDA  ($42),Y      ; b1 42
cf7b: STA  $41          ; 85 41
cf7d: RTS               ; 60 
cf7e: .byte $57   ;?
cf7f: LDX  #$a2         ; a2 a2
cf81: ORA  ($b1,X)      ; 01 b1
cf83: .byte $42   ;?
cf84: STA  $1251,X      ; 8d 51 12
cf87: INX               ; e8 
cf88: LDA  ($42),Y      ; b1 42
cf8a: STA  $1252,X      ; 8d 52 12
cf8d: .byte $97   ;?
cf8e: .byte $9f   ;?
cf8f: .byte $d7   ;?
cf90: LDX  #$4c         ; a2 4c
cf92: SED               ; f8 
cf93: .byte $cf   ;?
cf94: LDX  #$01         ; a2 01
cf96: LDA  ($42),Y      ; b1 42
cf98: PHA               ; 48 
cf99: INX               ; e8 
cf9a: LDA  ($42),Y      ; b1 42
cf9c: STA  $43          ; 85 43
cf9e: PLA               ; 68 
cf9f: STA  $42          ; 85 42
cfa1: JMP  ($0042)      ; 6c 42 00
cfa4: LDX  #$01         ; a2 01
cfa6: LDA  ($40),Y      ; b1 40
cfa8: CMP  #$05         ; c9 05
cfaa: BNE  $cfd3        ; d0 27
cfac: INX               ; e8 
cfad: LDA  ($40),Y      ; b1 40
cfaf: STA  $13ad,X      ; 8d ad 13
cfb2: INX               ; e8 
cfb3: LDA  ($40),Y      ; b1 40
cfb5: STA  $13ae,X      ; 8d ae 13
cfb8: LDX  #$06         ; a2 06
cfba: LDA  ($40),Y      ; b1 40
cfbc: STA  $13af,X      ; 8d af 13
cfbf: INX               ; e8 
cfc0: LDA  ($40),Y      ; b1 40
cfc2: STA  $13b0,X      ; 8d b0 13
cfc5: .byte $57   ;?
cfc6: LDX  #$a9         ; a2 a9
cfc8: PLP               ; 28 
cfc9: STA  $1251,X      ; 8d 51 12
cfcc: LDA  #$d1         ; a9 d1
cfce: STA  $1252,X      ; 8d 52 12
cfd1: BRA  $cf8d        ; 80 ba
cfd3: CMP  #$85         ; c9 85
cfd5: BNE  $cff2        ; d0 1b
cfd7: LDX  #$02         ; a2 02
cfd9: LDA  ($40),Y      ; b1 40
cfdb: PHA               ; 48 
cfdc: INX               ; e8 
cfdd: LDA  ($40),Y      ; b1 40
cfdf: PHA               ; 48 
cfe0: LDX  #$06         ; a2 06
cfe2: LDA  ($40),Y      ; b1 40
cfe4: STA  $44          ; 85 44
cfe6: INX               ; e8 
cfe7: LDA  ($40),Y      ; b1 40
cfe9: STA  $45          ; 85 45
cfeb: PLA               ; 68 
cfec: STA  $41          ; 85 41
cfee: PLA               ; 68 
cfef: STA  $40          ; 85 40
cff1: RTS               ; 60 
cff2: JSR  $1447        ; 20 47 14
cff5: .byte $87   ;?
cff6: .byte $9f   ;?
cff7: RTS               ; 60 
cff8: .byte $a7   ;?
cff9: .byte $9f   ;?
cffa: RTS               ; 60 
cffb: .byte $87   ;?
cffc: .byte $9f   ;?
cffd: RTS               ; 60 
cffe: LDA  ($6a)        ; b2 6a

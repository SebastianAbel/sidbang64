//----------------------------------------------------------
.pc = $7c "ZP variables" virtual
lineIdx:	.byte 0

//----------------------------------------------------------
.pc=$0801 "Basic Upstart Program"
:BasicUpstart($0810)

.var ui_bgColor = 0
.var ui_scColor = 0

//----------------------------------------------------------
.pc = $0810 "Main Program"

			sei
/*
			lda #ui_bgColor
			sta $d020
			lda #ui_scColor
			sta $d021
			lda #$00
			sta Memory.SetMemDest
			lda #$04
			sta Memory.SetMemDest+1
			ldx #$00
			ldy #$04
			lda #$20
			jsr Memory.SetMem

			ldx #$00
!:			lda credits,x
			sta $0400,x
			inx
			cpx #$28
			bne !-

			ldx #$0c
			jsr Memory.SetFGColor
*/			
			lda #$7f
			sta $dc0d
			sta $dd0d

			lda $dc0d
			lda $dd0d

			lda #$81
			sta $d01a

			lda #$35
			sta $01 

			lda #<irqHandler_plain  //this is how we set up
			sta $fffe  //the address of our interrupt code
			lda #>irqHandler_plain
			sta $ffff
			asl $d019

			jsr generateIrqPositions
			lda irqPositionH+39
			sta $d011
			lda irqPositionL+39
			sta $d012

			lda #$00
			sta lineIdx

			jsr music_init

!:			
			lda $d011
			and #$80
			beq !-
!:
			lda $d011
			and #$80
			bne !-
			cli	

mainloop_1:			
			jmp mainloop_1


generateIrqPositions:
			lda #$00
			sta $80
			lda #$00	// start-offset-l
			sta $81
			sta irqPositionL+15
			//sta irqPositionL+23		// 24x
			lda #$00	
			sta $82
			ora #$5b
			sta irqPositionH+15
			//sta irqPositionH+23		// 24x
			ldy #$00
gIP_loop1:			 
			clc
			lda $80
			adc #$80
			//adc #$00	// 24x
			sta $80
			lda $81
			adc #$13
			//adc #$0d	// 24x
			sta $81
			sta irqPositionL,y
			lda $82
			adc #$00
			sta $82
			clc
			ror
			ror
			ora #$5b
			sta irqPositionH,y
			iny
			cpy #15
			//cpy #23	// 24x
			bne gIP_loop1
			rts

.import source "memory.asm"

.pc = $0e00	"IRQ-handler"
irqHandler_plain:
			//dec $d020
			stx xTemp+1
			sty yTemp+1
			sta aTemp+1

			ldy lineIdx
			lda irqPositionL,y
			sta $d012
			lda irqPositionH,y
			sta $d011

			tya
			and #$01
			beq !+

			nop
			nop
			nop
			nop
			
			nop
			nop
			nop
			nop
			
			nop
			nop
			nop
			nop

			nop
			nop
			nop
			nop
!:
			iny
			cpy #16
			bne !+
			
			ldy #$00
!:			sty lineIdx

			jsr music_update

yTemp:		ldy #$00
xTemp:		ldx #$00
aTemp:		lda #$00
			asl $d019	// clear irq-request
			//inc $d020
			rti

			

.pc=$0f00	"IRQ position table" virtual
irqPositionL:
			.fill 32,0 
irqPositionH:
			.fill 32,0 

.pc=$1000	"music"
.import source "music.asm"

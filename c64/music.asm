music_init:
			jmp m_init
music_update:
// --- channel 1
			lda dataCh1
			sta dataPtr
			lda dataCh1+1
			sta dataPtr+1

			ldy #$00
			lda (dataPtr),y

			cmp #$00
			beq end_chnl1
			and #31
			tax

			lda voc1_kernelPosL,x
			sta kJsr1+1
			lda voc1_kernelPosH,x
			sta kJsr1+2
kJsr1:		jsr voc1_kernel0


end_chnl1:

			iny
			tya
			clc
			adc dataPtr
			sta dataCh1
			lda dataPtr+1
			adc #$00
			sta dataCh1+1

			//jmp preend_irq

// --- channel 2

			lda dataCh2
			sta dataPtr
			lda dataCh2+1
			sta dataPtr+1

			ldy #$00
			lda (dataPtr),y

			cmp #$00
			beq end_chnl2
			and #31
			tax

			lda voc2_kernelPosL,x
			sta kJsr2+1
			lda voc2_kernelPosH,x
			sta kJsr2+2
kJsr2:		jsr voc2_kernel0


end_chnl2:
			iny
			tya
			clc
			adc dataPtr
			sta dataCh2
			lda dataPtr+1
			adc #$00
			sta dataCh2+1

// --- channel 3

			lda dataCh3
			sta dataPtr
			lda dataCh3+1
			sta dataPtr+1

			ldy #$00
			lda (dataPtr),y

			cmp #$00
			beq end_chnl3
			and #31
			tax

			lda voc3_kernelPosL,x
			sta kJsr3+1
			lda voc3_kernelPosH,x
			sta kJsr3+2
kJsr3:		jsr voc3_kernel0


end_chnl3:
			iny
			tya
			clc
			adc dataPtr
			sta dataCh3
			lda dataPtr+1
			adc #$00
			sta dataCh3+1

preend_irq:
			inc subIdx
			lda subIdx
qnt_cmp:
			cmp bng_qsize
			bne end_irq
			lda #0
			sta subIdx
			jmp update_data
end_irq:
			rts

update_data:
			clc
			lda dataIdx1
			adc #$02
			sta dataIdx1
			bcc !+
			inc dataIdx1+1
!:			

			clc
			lda dataIdx2
			adc #$02
			sta dataIdx2
			bcc !+
			inc dataIdx2+1
!:

			clc
			lda dataIdx3
			adc #$02
			sta dataIdx3
			bcc !+
			inc dataIdx3+1
!:
			
			jmp frame_cmp		// comment out to enable the custom filter-changes

			inc	filterCtr
			lda filterCtr	
			cmp #48
			bne frame_cmp
			lda	#$00
			sta filterCtr

			ldx	filterCtr+1

			lda	filter_v,x
			sta	$d415
			lda	filter_v+1,x
			sta $d416
			lda	filter_v+2,x
			sta	$d417
			lda	filter_v+3,x
			sta	$d418

			txa
			clc
			adc #$04
			cmp #4*9
			bne !+
			lda #$00
!:			
			sta filterCtr+1

frame_cmp:
			clc
			lda frameCtr
			adc #$01
			sta frameCtr
			lda frameCtr+1
			adc #$00
			sta frameCtr+1
frh_cmp:	
			cmp bng_ticks+1
			bne !end+
			lda frameCtr
frl_cmp:	
			cmp bng_ticks
			bne !end+
			
			lda #$00
			sta frameCtr
			sta frameCtr+1

			inc patternCtr
			lda patternCtr
pat_cmp:	cmp bng_ptc	// number of patterns
			bne !+
			lda #$00
			sta patternCtr
!:			asl
			tay
		
			lda ch_idx_ptr1,y
			sta dataIdx1
			lda ch_idx_ptr1+1,y
			sta dataIdx1+1

			lda ch_idx_ptr2,y
			sta dataIdx2
			lda ch_idx_ptr2+1,y
			sta dataIdx2+1
			
			lda ch_idx_ptr3,y
			sta dataIdx3
			lda ch_idx_ptr3+1,y
			sta dataIdx3+1						

!end:
			ldy #0
			clc
			lda (dataIdx1),y
			adc ch_ptr
			sta dataCh1
			iny
			lda (dataIdx1),y
			adc ch_ptr+1
			sta dataCh1+1
			dey
			clc
			lda (dataIdx2),y
			adc ch_ptr+2
			sta dataCh2
			iny
			lda (dataIdx2),y
			adc ch_ptr+3
			sta dataCh2+1
			dey
			clc
			lda (dataIdx3),y
			adc ch_ptr+4
			sta dataCh3
			iny
			lda (dataIdx3),y
			adc ch_ptr+5
			sta dataCh3+1

			rts

m_init:
			lda #$00
			sta subIdx

			sta frameCtr
			sta frameCtr+1
			sta dataPtr
			sta dataPtr+1
			sta patternCtr

			lda ch_idx_ptr1
			sta dataIdx1
			lda ch_idx_ptr1+1
			sta dataIdx1+1

			lda ch_idx_ptr2
			sta dataIdx2
			lda ch_idx_ptr2+1
			sta dataIdx2+1
			
			lda ch_idx_ptr3
			sta dataIdx3
			lda ch_idx_ptr3+1
			sta dataIdx3+1			

			ldy #0
			clc
			lda (dataIdx1),y
			adc ch_ptr
			sta dataCh1
			iny
			lda (dataIdx1),y
			adc ch_ptr+1
			sta dataCh1+1
			dey
			clc
			lda (dataIdx2),y
			adc ch_ptr+2
			sta dataCh2
			iny
			lda (dataIdx2),y
			adc ch_ptr+3
			sta dataCh2+1
			dey
			clc
			lda (dataIdx3),y
			adc ch_ptr+4
			sta dataCh3
			iny
			lda (dataIdx3),y
			adc ch_ptr+5
			sta dataCh3+1

			lda #$00
			ldx #$17
!:			sta $d400,x
			dex
			bpl !-

			ldx #$00
			stx	filterCtr
			lda	filter_v,x
			sta	$d415
			lda	filter_v+1,x
			sta $d416
			lda	filter_v+2,x
			sta	$d417
			lda	filter_v+3,x
			sta	$d418

			ldx #$04
			stx	filterCtr+1
			rts			


.import source "voc1_kernels.asm"
.import source "voc2_kernels.asm"
.import source "voc3_kernels.asm"	

.pc =$1c00 "song data"
credits:
//			.text	"0123456789012345678901234567890123456789"
			.text	"'promofiepen' / w4rp8 / 2021 / 8580     "

filter_v:	.byte	0 & 7, 0>>3
			.byte	$00, $0f

			.byte	250 & 7, 250>>3
			.byte	$81, $1f
			
			.byte	113 & 7, 113>>3
			.byte	$d4, $1f
			
			.byte	481 & 7, 481>>3
			.byte	$84, $5f
			
			.byte	1024 & 7, 1024>>3
			.byte	$a1, $1f

			.byte	327 & 7, 327>>3
			.byte	$f3, $1f

			.byte	256 & 7, 256>>3
			.byte	$e2, $4f

			.byte	1297 & 7, 1297>>3
			.byte	$e4, $6f

			.byte	203 & 7, 203>>3
			.byte	$a1, $1f


ch_ptr:		.word ch1, ch2, ch3
.import source "data/promofiepen.asm"

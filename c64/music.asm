//----------------------------------------------------------
.pc = $02 "ZP variables" virtual
subIdx:		.byte 0
frameCtr:	.word 0
patternCtr:	.word 0
dataPtr:	.word 0
dataCh1:	.word 0
dataCh2:	.word 0
dataCh3:	.word 0
dataIdx1:	.word 0
dataIdx2:	.word 0
dataIdx3:	.word 0

idx1:		.byte 0
idx2:		.byte 0
filterCtr:	.byte 0, 0
ffreqStore:	.byte 0, 0
ffreqAdd:	.byte 0, 0

.pc=$1000	"music player"
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
			
			//jmp frame_cmp		// comment out to enable the custom filter-changes

			inc	filterCtr
			lda filterCtr	
			cmp filter_patch_length
			beq restart_filter			
			clc
			lda ffreqStore
			adc ffreqAdd
			sta ffreqStore
			//lsr
			//lsr
			//lsr
			//lsr
			//lsr
			sta	$d415
			lda ffreqStore+1
			adc ffreqAdd+1
			sta ffreqStore+1
			sta	$d416

			jmp frame_cmp

restart_filter:
			lda	#$00
			sta filterCtr

			ldx	filterCtr+1
			lda	filter_v,x
			sta ffreqStore
			lsr
			lsr
			lsr
			lsr
			lsr
			sta	$d415
			lda	filter_v+1,x
			sta ffreqStore+1
			sta $d416
			lda	filter_v+2,x
			sta	$d417
			lda	filter_v+3,x
			sta	$d418
			lda	filter_v+4,x
			sta ffreqAdd
			lda	filter_v+5,x
			sta ffreqAdd+1

			txa
			clc
			adc #$06
			cmp filter_patches
			bne !+
			lda #$00
!:			
			sta filterCtr+1


frame_cmp:
			clc
			lda frameCtr
			adc #$01
			sta frameCtr
			bcc !+
			inc frameCtr+1
!:			
frh_cmp:	
			lda frameCtr+1
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
			dec patternCtr
			//lda #$00
			//sta patternCtr
			lda patternCtr
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
			lda (dataIdx1),y
			sta dataCh1
			lda (dataIdx2),y
			sta dataCh2
			lda (dataIdx3),y
			sta dataCh3
			iny
			lda (dataIdx1),y
			sta dataCh1+1
			lda (dataIdx2),y
			sta dataCh2+1
			lda (dataIdx3),y
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
			lda (dataIdx1),y
			sta dataCh1
			lda (dataIdx2),y
			sta dataCh2
			lda (dataIdx3),y
			sta dataCh3
			iny
			lda (dataIdx1),y
			sta dataCh1+1
			lda (dataIdx2),y
			sta dataCh2+1
			lda (dataIdx3),y
			sta dataCh3+1

			lda #$00
			ldx #$17
!:			sta $d400,x
			dex
			bpl !-

			ldx #$00
			stx	filterCtr

			lda	filter_v,x
			sta ffreqStore
			lsr
			lsr
			lsr
			lsr
			lsr
			sta	$d415
			lda	filter_v+1,x
			sta ffreqStore+1
			sta $d416
			lda	filter_v+2,x
			sta	$d417
			lda	filter_v+3,x
			sta	$d418
			lda	filter_v+4,x
			sta ffreqAdd
			lda	filter_v+5,x
			sta ffreqAdd+1

			ldx #$06
			stx	filterCtr+1
			rts			


.import source "voc1_kernels.asm"
.import source "voc2_kernels.asm"
.import source "voc3_kernels.asm"	

.pc =$1b80 "song data"
credits:
//			.text	"0123456789012345678901234567890123456789"
			//.text	"sidbang64replay v0.7 / w4rp8 / 2021     "
			.text	"jam'n'flute (8580)      w4rp8/pht - 2021"

.import source "data/bngfilter.asm"
.import source "data/bng.asm"

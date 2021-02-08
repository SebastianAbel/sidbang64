
voc1_kernelPosL:	
			.byte <voc1_kernel0, <voc1_kernel1, <voc1_kernel2, <voc1_kernel3
			.byte <voc1_kernel4, <voc1_kernel5, <voc1_kernel6, <voc1_kernel7
			.byte <voc1_kernel8, <voc1_kernel9, <voc1_kernel10, <voc1_kernel11
			.byte <voc1_kernel12, <voc1_kernel13, <voc1_kernel14, <voc1_kernel15

			.byte <voc1_kernel16, <voc1_kernel17, <voc1_kernel18, <voc1_kernel19
			.byte <voc1_kernel20, <voc1_kernel21, <voc1_kernel22, <voc1_kernel23
			.byte <voc1_kernel24, <voc1_kernel25, <voc1_kernel26, <voc1_kernel27
			.byte <voc1_kernel28, <voc1_kernel29, <voc1_kernel30, <voc1_kernel31

voc1_kernelPosH:	
			.byte >voc1_kernel0, >voc1_kernel1, >voc1_kernel2, >voc1_kernel3
			.byte >voc1_kernel4, >voc1_kernel5, >voc1_kernel6, >voc1_kernel7
			.byte >voc1_kernel8, >voc1_kernel9, >voc1_kernel10, >voc1_kernel11
			.byte >voc1_kernel12, >voc1_kernel13, >voc1_kernel14, >voc1_kernel15

			.byte >voc1_kernel16, >voc1_kernel17, >voc1_kernel18, >voc1_kernel19
			.byte >voc1_kernel20, >voc1_kernel21, >voc1_kernel22, >voc1_kernel23
			.byte >voc1_kernel24, >voc1_kernel25, >voc1_kernel26, >voc1_kernel27
			.byte >voc1_kernel28, >voc1_kernel29, >voc1_kernel30, >voc1_kernel31

//----------------------------------------------------------

voc1_kernel0:
			rts
voc1_kernel1:
			iny
			lda (dataPtr),y
			sta $d400 
			rts

voc1_kernel2:
			iny
			lda (dataPtr),y
			sta $d401 
			rts			

voc1_kernel3:
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d400
			sta $d401 			
			rts





voc1_kernel4:
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403 
			rts

voc1_kernel5:
			iny
			lda (dataPtr),y
			sta $d400
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403 
			rts

voc1_kernel6:
			iny
			lda (dataPtr),y
			sta $d401
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403 
			rts

voc1_kernel7:
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d400
			sta $d401			
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403 
			rts






voc1_kernel8:
			iny
			lda (dataPtr),y
			sta $d404
			rts
voc1_kernel9:
			iny
			lda (dataPtr),y
			sta $d400
			iny
			lda (dataPtr),y
			sta $d404			 
			rts

voc1_kernel10:
			iny
			lda (dataPtr),y
			sta $d401
			iny
			lda (dataPtr),y
			sta $d404			
			rts			

voc1_kernel11:
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d400
			sta $d401
			iny
			lda (dataPtr),y
			sta $d404			 			
			rts





voc1_kernel12:
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403
			iny
			lda (dataPtr),y
			sta $d404			 
			rts

voc1_kernel13:
			iny
			lda (dataPtr),y
			sta $d400
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403
			iny
			lda (dataPtr),y
			sta $d404			 
			rts

voc1_kernel14:
			iny
			lda (dataPtr),y
			sta $d401
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403
			iny
			lda (dataPtr),y
			sta $d404			 
			rts

voc1_kernel15:
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d400
			sta $d401			
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403
			iny
			lda (dataPtr),y
			sta $d404			 
			rts



voc1_kernel16:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406	
			rts
voc1_kernel17:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			sta $d400 
			rts

voc1_kernel18:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			sta $d401 
			rts			

voc1_kernel19:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d400
			sta $d401 			
			rts



voc1_kernel20:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403 
			rts

voc1_kernel21:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			sta $d400
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403 
			rts

voc1_kernel22:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			sta $d401
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403 
			rts

voc1_kernel23:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d400
			sta $d401			
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403 
			rts






voc1_kernel24:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			sta $d404
			rts
voc1_kernel25:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			sta $d400
			iny
			lda (dataPtr),y
			sta $d404			 
			rts

voc1_kernel26:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			sta $d401 
			iny
			lda (dataPtr),y
			sta $d404			
			rts			

voc1_kernel27:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d400
			sta $d401
			iny
			lda (dataPtr),y
			sta $d404			 			
			rts





voc1_kernel28:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403
			iny
			lda (dataPtr),y
			sta $d404			 
			rts

voc1_kernel29:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			sta $d400
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403
			iny
			lda (dataPtr),y
			sta $d404			 
			rts

voc1_kernel30:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			sta $d401
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403
			iny
			lda (dataPtr),y
			sta $d404			 
			rts

voc1_kernel31:
			iny
			lda (dataPtr),y
			sta $d405
			iny
			lda (dataPtr),y
			sta $d406
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d400
			sta $d401			
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d402
			sta $d403
			iny
			lda (dataPtr),y
			sta $d404			 
			rts






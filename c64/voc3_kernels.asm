
voc3_kernelPosL:	
			.byte <voc3_kernel0, <voc3_kernel1, <voc3_kernel2, <voc3_kernel3
			.byte <voc3_kernel4, <voc3_kernel5, <voc3_kernel6, <voc3_kernel7
			.byte <voc3_kernel8, <voc3_kernel9, <voc3_kernel10, <voc3_kernel11
			.byte <voc3_kernel12, <voc3_kernel13, <voc3_kernel14, <voc3_kernel15

			.byte <voc3_kernel16, <voc3_kernel17, <voc3_kernel18, <voc3_kernel19
			.byte <voc3_kernel20, <voc3_kernel21, <voc3_kernel22, <voc3_kernel23
			.byte <voc3_kernel24, <voc3_kernel25, <voc3_kernel26, <voc3_kernel27
			.byte <voc3_kernel28, <voc3_kernel29, <voc3_kernel30, <voc3_kernel31

voc3_kernelPosH:	
			.byte >voc3_kernel0, >voc3_kernel1, >voc3_kernel2, >voc3_kernel3
			.byte >voc3_kernel4, >voc3_kernel5, >voc3_kernel6, >voc3_kernel7
			.byte >voc3_kernel8, >voc3_kernel9, >voc3_kernel10, >voc3_kernel11
			.byte >voc3_kernel12, >voc3_kernel13, >voc3_kernel14, >voc3_kernel15

			.byte >voc3_kernel16, >voc3_kernel17, >voc3_kernel18, >voc3_kernel19
			.byte >voc3_kernel20, >voc3_kernel21, >voc3_kernel22, >voc3_kernel23
			.byte >voc3_kernel24, >voc3_kernel25, >voc3_kernel26, >voc3_kernel27
			.byte >voc3_kernel28, >voc3_kernel29, >voc3_kernel30, >voc3_kernel31

//----------------------------------------------------------

voc3_kernel0:
			rts
voc3_kernel1:
			iny
			lda (dataPtr),y
			sta $d40e 
			rts

voc3_kernel2:
			iny
			lda (dataPtr),y
			sta $d40f 
			rts			

voc3_kernel3:
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d40e
			sta $d40f 			
			rts





voc3_kernel4:
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411 
			rts

voc3_kernel5:
			iny
			lda (dataPtr),y
			sta $d40e
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411 
			rts

voc3_kernel6:
			iny
			lda (dataPtr),y
			sta $d40f
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411 
			rts

voc3_kernel7:
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d40e
			sta $d40f			
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411 
			rts






voc3_kernel8:
			iny
			lda (dataPtr),y
			sta $d412
			rts
voc3_kernel9:
			iny
			lda (dataPtr),y
			sta $d40e
			iny
			lda (dataPtr),y
			sta $d412			 
			rts

voc3_kernel10:
			iny
			lda (dataPtr),y
			sta $d40f
			iny
			lda (dataPtr),y
			sta $d412			
			rts			

voc3_kernel11:
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d40e
			sta $d40f
			iny
			lda (dataPtr),y
			sta $d412			 			
			rts





voc3_kernel12:
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411
			iny
			lda (dataPtr),y
			sta $d412			 
			rts

voc3_kernel13:
			iny
			lda (dataPtr),y
			sta $d40e
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411
			iny
			lda (dataPtr),y
			sta $d412			 
			rts

voc3_kernel14:
			iny
			lda (dataPtr),y
			sta $d40f
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411
			iny
			lda (dataPtr),y
			sta $d412			 
			rts

voc3_kernel15:
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d40e
			sta $d40f			
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411
			iny
			lda (dataPtr),y
			sta $d412			 
			rts



voc3_kernel16:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414	
			rts
voc3_kernel17:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			sta $d40e 
			rts

voc3_kernel18:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			sta $d40f 
			rts			

voc3_kernel19:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d40e
			sta $d40f 			
			rts



voc3_kernel20:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411 
			rts

voc3_kernel21:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			sta $d40e
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411 
			rts

voc3_kernel22:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			sta $d40f
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411 
			rts

voc3_kernel23:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d40e
			sta $d40f			
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411 
			rts






voc3_kernel24:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			sta $d412
			rts
voc3_kernel25:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			sta $d40e
			iny
			lda (dataPtr),y
			sta $d412			 
			rts

voc3_kernel26:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			sta $d40f 
			iny
			lda (dataPtr),y
			sta $d412			
			rts			

voc3_kernel27:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d40e
			sta $d40f
			iny
			lda (dataPtr),y
			sta $d412			 			
			rts





voc3_kernel28:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411
			iny
			lda (dataPtr),y
			sta $d412			 
			rts

voc3_kernel29:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			sta $d40e
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411
			iny
			lda (dataPtr),y
			sta $d412			 
			rts

voc3_kernel30:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			sta $d40f
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411
			iny
			lda (dataPtr),y
			sta $d412			 
			rts

voc3_kernel31:
			iny
			lda (dataPtr),y
			sta $d413
			iny
			lda (dataPtr),y
			sta $d414
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d40e
			sta $d40f			
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d410
			sta $d411
			iny
			lda (dataPtr),y
			sta $d412			 
			rts






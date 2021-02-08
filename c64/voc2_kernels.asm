
voc2_kernelPosL:	
			.byte <voc2_kernel0, <voc2_kernel1, <voc2_kernel2, <voc2_kernel3
			.byte <voc2_kernel4, <voc2_kernel5, <voc2_kernel6, <voc2_kernel7
			.byte <voc2_kernel8, <voc2_kernel9, <voc2_kernel10, <voc2_kernel11
			.byte <voc2_kernel12, <voc2_kernel13, <voc2_kernel14, <voc2_kernel15

			.byte <voc2_kernel16, <voc2_kernel17, <voc2_kernel18, <voc2_kernel19
			.byte <voc2_kernel20, <voc2_kernel21, <voc2_kernel22, <voc2_kernel23
			.byte <voc2_kernel24, <voc2_kernel25, <voc2_kernel26, <voc2_kernel27
			.byte <voc2_kernel28, <voc2_kernel29, <voc2_kernel30, <voc2_kernel31

voc2_kernelPosH:	
			.byte >voc2_kernel0, >voc2_kernel1, >voc2_kernel2, >voc2_kernel3
			.byte >voc2_kernel4, >voc2_kernel5, >voc2_kernel6, >voc2_kernel7
			.byte >voc2_kernel8, >voc2_kernel9, >voc2_kernel10, >voc2_kernel11
			.byte >voc2_kernel12, >voc2_kernel13, >voc2_kernel14, >voc2_kernel15

			.byte >voc2_kernel16, >voc2_kernel17, >voc2_kernel18, >voc2_kernel19
			.byte >voc2_kernel20, >voc2_kernel21, >voc2_kernel22, >voc2_kernel23
			.byte >voc2_kernel24, >voc2_kernel25, >voc2_kernel26, >voc2_kernel27
			.byte >voc2_kernel28, >voc2_kernel29, >voc2_kernel30, >voc2_kernel31

//----------------------------------------------------------

voc2_kernel0:
			rts
voc2_kernel1:
			iny
			lda (dataPtr),y
			sta $d407 
			rts

voc2_kernel2:
			iny
			lda (dataPtr),y
			sta $d408 
			rts			

voc2_kernel3:
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d407
			sta $d408 			
			rts





voc2_kernel4:
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a 
			rts

voc2_kernel5:
			iny
			lda (dataPtr),y
			sta $d407
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a 
			rts

voc2_kernel6:
			iny
			lda (dataPtr),y
			sta $d408
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a 
			rts

voc2_kernel7:
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d407
			sta $d408			
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a 
			rts






voc2_kernel8:
			iny
			lda (dataPtr),y
			sta $d40b
			rts
voc2_kernel9:
			iny
			lda (dataPtr),y
			sta $d407
			iny
			lda (dataPtr),y
			sta $d40b			 
			rts

voc2_kernel10:
			iny
			lda (dataPtr),y
			sta $d408
			iny
			lda (dataPtr),y
			sta $d40b			
			rts			

voc2_kernel11:
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d407
			sta $d408
			iny
			lda (dataPtr),y
			sta $d40b			 			
			rts





voc2_kernel12:
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a
			iny
			lda (dataPtr),y
			sta $d40b			 
			rts

voc2_kernel13:
			iny
			lda (dataPtr),y
			sta $d407
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a
			iny
			lda (dataPtr),y
			sta $d40b			 
			rts

voc2_kernel14:
			iny
			lda (dataPtr),y
			sta $d408
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a
			iny
			lda (dataPtr),y
			sta $d40b			 
			rts

voc2_kernel15:
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d407
			sta $d408			
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a
			iny
			lda (dataPtr),y
			sta $d40b			 
			rts



voc2_kernel16:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d	
			rts
voc2_kernel17:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			sta $d407 
			rts

voc2_kernel18:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			sta $d408 
			rts			

voc2_kernel19:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d407
			sta $d408 			
			rts



voc2_kernel20:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a 
			rts

voc2_kernel21:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			sta $d407
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a 
			rts

voc2_kernel22:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			sta $d408
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a 
			rts

voc2_kernel23:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d407
			sta $d408			
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a 
			rts






voc2_kernel24:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			sta $d40b
			rts
voc2_kernel25:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			sta $d407
			iny
			lda (dataPtr),y
			sta $d40b			 
			rts

voc2_kernel26:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			sta $d408 
			iny
			lda (dataPtr),y
			sta $d40b			
			rts			

voc2_kernel27:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d407
			sta $d408
			iny
			lda (dataPtr),y
			sta $d40b			 			
			rts





voc2_kernel28:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a
			iny
			lda (dataPtr),y
			sta $d40b			 
			rts

voc2_kernel29:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			sta $d407
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a
			iny
			lda (dataPtr),y
			sta $d40b			 
			rts

voc2_kernel30:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			sta $d408
			iny
			lda (dataPtr),y
			tax 
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a
			iny
			lda (dataPtr),y
			sta $d40b			 
			rts

voc2_kernel31:
			iny
			lda (dataPtr),y
			sta $d40c
			iny
			lda (dataPtr),y
			sta $d40d
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d407
			sta $d408			
			iny
			lda (dataPtr),y
			tax
			iny
			lda (dataPtr),y
			stx $d409
			sta $d40a
			iny
			lda (dataPtr),y
			sta $d40b			 
			rts






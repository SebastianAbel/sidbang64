.macro SetMem(start, length, value)
{
			lda #<start
			sta Memory.SetMemDest
			lda #>start
			sta Memory.SetMemDest+1
			ldx #<length
			ldy #>length
			lda #value
			jsr Memory.SetMem
}

.macro SetMemX(start, length)
{
			lda #<start
			sta Memory.SetMemDest
			lda #>start
			sta Memory.SetMemDest+1
			txa
			ldx #<length
			ldy #>length
			jsr Memory.SetMem
}

.macro MemCpy_small(source, dest, length)
{
			lda #<source
			sta Memory.cms_s+1
			lda #>source
			sta Memory.cms_s+2
			lda #<dest
			sta Memory.cms_d+1
			lda #>dest
			sta Memory.cms_d+2			
			ldx #<length
			jsr Memory.CpyMemSmall
}

Memory: {
//----------------------------------------------------------
ClearScreen:
			:SetMem($0400, 40*25, $20)
			rts

//----------------------------------------------------------	
// x: color
SetFGColor:	:SetMemX($d800, 40*25)
			rts

//----------------------------------------------------------	
// a: value
// x: lengthLow
// y: lengthHigh
// SetMemDest: destination
SetMem:		pha
			lda SetMemDest
			sta loop1+1
			sta loop2+1
			lda SetMemDest+1
			sta loop1+2
			sta loop2+2
			stx fillrest+1
			pla
			
			cpy #$00
			beq fillrest
			sty ctr
			ldx #$00
			ldy #$00
loop1:		sta $ffff,y
			iny
			dex
			cpx #$00
			bne loop1
			inc loop1+2
			inc loop2+2
			dec ctr
			ldy ctr
			cpy #$00
			bne loop1-2
			
fillrest:	ldx #00
			cpx #$00
			beq end
			ldy #$00
loop2:		sta $ffff,y
			iny
			dex
			cpx #$00
			bne loop2
end:
			rts
ctr:		.byte 0			
SetMemDest:	.byte 0, 0


//----------------------------------------------------------	
// x: length
// y: lengthHigh
// SetMemDest: destination
CpyMemSmall:	
			ldy #$00
cms_loop:			
cms_s:		lda $ffff,y
cms_d:		sta $ffff,y
			iny
			dex
			bne cms_loop
			rts
}
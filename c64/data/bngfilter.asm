filter_patch_length:	
			.byte 64

filter_patches:	
			.byte 6*32	// min = 6, max = 6*32

filter_v:
			//		freq low , freq high
			//		reso|voice , filtermode | volume
			//		freq low add, freq high add

			// 0
			.byte	256<<5, 256>>3
			.byte	$82, $4c
			.byte	32 , 0
			.byte	356<<5, 356>>3
			.byte	$c2, $4d
			.byte	128, 0

			.byte	250<<5, 250>>3
			.byte	$81, $1e
			.byte	0, 1
			.byte	350<<5, 350>>3
			.byte	$c1, $1f
			.byte	0, 4

			.byte	313<<5, 313>>3
			.byte	$d4, $1f
			.byte	0, 1
			.byte	413<<5, 413>>3
			.byte	$e4, $1f
			.byte	128, 0

			.byte	481<<5, 481>>3
			.byte	$84, $5f
			.byte	128, 1			
			.byte	381<<5, 381>>3
			.byte	$94, $5f
			.byte	64, 1

			// 1
			.byte	1014<<5, 1014>>3
			.byte	$a1, $1e
			.byte 	4, 0
			.byte	924<<5, 924>>3
			.byte	$e1, $1f
			.byte 	16, 0

			.byte	218<<5, 218>>3
			.byte	$84, $5e		
			.byte 	0, 2
			.byte	118<<5, 118>>3
			.byte	$a4, $5f
			.byte 	0, 6

			.byte	427<<5, 427>>3
			.byte	$c3, $1d
			.byte 	32, 0
			.byte	100<<5, 100>>3
			.byte	$d3, $1e
			.byte 	0, 2

			.byte	512<<5, 512>>3
			.byte	$e2, $4f
			.byte 	4, 0
			.byte	412<<5, 412>>3
			.byte	$f2, $4f
			.byte 	16, 0

			// 2
			.byte	800<<5, 800>>3
			.byte	$d4, $3f
			.byte 	128, 0
			.byte	900<<5, 900>>3
			.byte	$e4, $3f
			.byte 	64, 0

			.byte	203<<5, 203>>3
			.byte	$a1, $1e
			.byte 	0, 1
			.byte	403<<5, 403>>3
			.byte	$b1, $1f
			.byte 	0, 4

			.byte	827<<5, 827>>3
			.byte	$75, $3d
			.byte 	0, 2
			.byte	890<<5, 890>>3
			.byte	$25, $3e
			.byte 	0, 1

			.byte	712<<5, 712>>3
			.byte	$a1, $1f
			.byte 	32, 2
			.byte	812<<5, 812>>3
			.byte	$c1, $1f
			.byte 	64, 4

			// 3
			.byte	113<<5, 113>>3
			.byte	$d4, $1f
			.byte 	0, 1
			.byte	213<<5, 213>>3
			.byte	$e4, $1f
			.byte 	0, 2

			.byte	227<<5, 227>>3
			.byte	$83, $1d
			.byte 	0, 900>>3
			.byte	327<<5, 327>>3
			.byte	$93, $1e
			.byte 	0, 11

			.byte	512<<5, 512>>3
			.byte	$e2, $4f
			.byte 	0, 1020>>3
			.byte	512<<5, 512>>3
			.byte	$f2, $4f
			.byte 	128, 0

			.byte	750<<5, 750>>3
			.byte	$b4, $3e
			.byte 	0, 2
			.byte	770<<5, 770>>3
			.byte	$e4, $3f
			.byte 	0, 3

		LDA		#4
		STA		P
		LDA		#3
		STA		Q
		LDA		P
		MUL		Q
		COMP	#10
		JGT		THEN
		LDA		#100
		STA		P
		LDA		#200
		STA		Q
		J		OUT
THEN	LDA		#1
		STA		P
		LDA		#2
		MUL		P
		LDA		#2
		MUL		P
		STA		Q
OUT		LDA		P
		ADD		Q
		STA		RES
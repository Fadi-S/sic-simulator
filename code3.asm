        LDT     #600
        LDX     #0
        LDS     #3
LOOP    LDA     #3
        MUL     Y,X
        STA     Y,X
        ADDR    S,X
        COMPR   X,T
        JLT     LOOP
Y       RESW    200
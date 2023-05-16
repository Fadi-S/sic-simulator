        LDA     #10
        STA     Q
        LDS     #3
        LDT     #4
LOOP    ADD     Q
        MULR    S,T
        STA     Q
        COMP    #20
        JEQ     LOOP
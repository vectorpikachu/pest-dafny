include "hello.dfy"

abstract module {:Attribute1 * ==> * :} AModule {
    /*
     * This is a comment
     */
    import opened A
    export  provides *
}

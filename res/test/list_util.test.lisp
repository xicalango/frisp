
(include "../res/list_util.lisp")

(define test-seq (lambda () 
    (define expected (list 10 9 8 7 6 5 4 3 2 1))
    (define actual (seq 10))
    (assert-eq expected actual)
))

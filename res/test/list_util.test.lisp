
(include "../res/list_util.lisp")

(define test-seq (lambda () 
    (define expected (list 10 9 8 7 6 5 4 3 2 1))
    (define actual (seq 10))
    (assert-eq expected actual)
))

(define test-reverse (lambda () 
    (define expected (list 1 2 3 4 5 6 7 8 9 10))
    (define actual (reverse (seq 10)))
    (assert-eq expected actual)
))

(define test-reduce (lambda ()
    (define expected 24)
    (define actual (reduce * 1 (list 2 3 4)))
    (assert-eq expected actual)
))

(define test-sum-all (lambda ()
    (define expected 6)
    (define actual (sum-all 0 (list 1 2 3)))
    (assert-eq expected actual)
))

(define test-map (lambda ()
    (define add1 (lambda (a) (+ a 1)))

    (define expected (list 2 3 4))
    (define actual (map add1 (list 1 2 3)))
    (assert-eq expected actual)
))

(define test-for-each (lambda ()
    (for-each 
        (lambda (v) (assert (< v 6)))
        (list 1 2 3 4 5)
    )
))

(define test-nth (lambda ()
    (define expected 5)
    (define actual (nth 5 (seq 10)))
    (assert-eq expected actual)
))

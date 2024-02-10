
(define test-string-length (lambda () 
    (define expected 5)
    (define actual (length "hello"))
    (assert-eq expected actual)
))

(define test-string-split (lambda ()
    (define expected (list "a" "b" "c"))
    (define actual (str-split "a,b,c" ","))
    (assert-eq expected actual)
))

(define test-string-lines (lambda ()
    (define expected (list "a" "b" "c"))
    (define actual (str-lines "a
b
c"))
    (assert-eq expected actual)
))

(define test-to-string (lambda ()
    (define expected (list "a" "1" "1.2"))
    (define actual (to-string "a" 1 1.2))
    (assert-eq expected actual)
))

(define test-to-string-single-arg (lambda ()
    (define expected "42")
    (define actual (to-string 42))
    (assert-eq expected actual)
))

(define test-string-concat (lambda ()
    (define expected "abc")
    (define actual (str-concat "a" "b" "c"))
    (assert-eq expected actual)
))

(define test-string-join (lambda ()
    (define expected "a,b,c")
    (define actual (str-join "," (list "a" "b" "c")))
    (assert-eq expected actual)
))

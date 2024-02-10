(define gcd-ui (lambda () 
    (print "Please enter first number:")
    (define a (parse-int (read-line)))

    (print "Please enter second number:")
    (define b (parse-int (read-line)))

    (define res (gcd a b))

    (print "The GCD of " a " and " b " is " res)
))

(define gcdui (lambda () 
    (print "Please enter first number:")
    (define a (parseInt (readLine)))

    (print "Please enter second number:")
    (define b (parseInt (readLine)))

    (define res (gcd a b))

    (print "The GCD of " a " and " b " is " res)
))

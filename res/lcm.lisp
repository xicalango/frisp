(define lcm (lambda (a b)
  (/ 
    (abs (* a b))
    (gcd a b)
  )
))
    

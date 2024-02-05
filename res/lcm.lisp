(define lcm (lambda (a b)
  (div 
    (abs (mul a b))
    (gcd a b)
  )
))
    

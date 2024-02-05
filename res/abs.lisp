(define abs (lambda (a)
  (if (lt a 0)
    (sub 0 a)
    (a)
  )
))

(define lcm (lambda (a b)
  (div 
    (abs (mul a b))
    (gcd a b)
  )
))
    

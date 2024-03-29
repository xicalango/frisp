(define reduce (lambda (fn accu elements)
  (if (endp elements)
    (accu)
    (reduce fn (fn accu (car elements)) (cdr elements))
  )
))

(define reverse (lambda (l)
  (if (endp l)
    ()
    (concatenate (reverse (cdr l)) (list (car l)))
  )
))

(define sum-all (lambda (accu elements)
  (if (endp elements)
    (accu)
    (sum-all (+ accu (car elements)) (cdr elements))
  )
))

(define concatenate (lambda (l1 l2)
  (if (endp l1)
    (l2)
    (cons (car l1) (concatenate (cdr l1) l2))
  )
))

(define seq (lambda (n)
  (if (== n 0)
    ()
    (cons n (seq (- n 1)))
  )
))

(define map (lambda (mapper l)
    (if (endp l)
        ()
        (cons (mapper (car l)) (map mapper (cdr l)))
    )
))

(define for-each (lambda (body l)
    (if (endp l)
        ()
        (progn
            (body (car l))
            (for-each body (cdr l))
        )
    )
))

(define nth (lambda (n l)
  (if (== n 0)
    (car l)
    (nth (- n 1) (cdr l))
  )
))

(define index-of (lambda (val l)
  (define index-of-w (lambda (n val l)
    (if (endp l)
      ()
      (if (== val (car l))
        (n)
        (index-of-w (+ n 1) val (cdr l))
      )
    )
  ))
  (index-of-w 0 val l)
))

(define find-by-key (lambda (key l)
  (if (endp l)
    ()
    (if (== key (car (car l)))
      (car l)
      (find-by-key key (cdr l))
    )
  )
))

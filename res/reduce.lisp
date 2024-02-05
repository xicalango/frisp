(define reduce (lambda (fn accu elements)
  (if (endp elements)
    (accu)
    (reduce fn (fn accu (car elements)) (cdr elements))
  )
))

(define sum-all (lambda (accu elements)
  (if (endp elements)
    (accu)
    (sum-all (add accu (car elements)) (cdr elements))
  )
))
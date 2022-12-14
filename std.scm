(define (map fn ls)
  (if (empty? ls)
    '()
    (cons (fn (car ls)) (map fn (cdr ls)))))

(define (filter fn ls)
  (if (empty? ls)
    '()
    (if (fn (car ls))
      (cons (car ls) (filter fn (cdr ls)))
      (filter fn (cdr ls)))))

(define (reduce fn ls)
  (if (empty? (cdr ls))
    (car ls)
    (fn (car ls) (reduce fn (cdr ls)))))

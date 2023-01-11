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

(define (reduce fn init ls)
  (if (empty? ls)
    init
    (fn (car ls) (reduce fn init (cdr ls)))))

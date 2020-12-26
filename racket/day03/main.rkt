#lang racket
(require racket/stream)

(struct forest (width height trees) #:transparent)

(define (get map p)
  (define pos (cons (modulo (car p) (forest-width map))
                    (modulo (cdr p) (forest-height map))))
 
  (set-member? (forest-trees map) pos))

(define (parse-map)
  (define s (mutable-set))
  (define width 0)
  (define height 0)
  (for ([line (port->lines)]
        [row (in-naturals)])
    (set! height (max height row))
    (for ([elem line]
          [col (in-naturals)])
      (set! width (max width col))
      (cond [(equal? #\# elem) (set-add! s (cons col row))])))
  (forest (add1 width) (add1 height) s))

(define (all-positions x y dx dy)
  (stream-cons (cons x y) (all-positions (+ x dx) (+ y dy) dx dy)))

(define (positions max-y dx dy)
  (for/list ([s (in-stream (all-positions dx dy dx dy))] #:break (>= (cdr s) max-y)) s))

(define (solve m slope)
  (define pos (positions (forest-height m)
                         (car slope)
                         (cdr slope)))
  (count (lambda (x) x) (map (lambda (p) (get m p)) pos)))

(module* main #f
  (define m (parse-map))
  (define pos (positions (forest-height m) 3 1))
  (println (solve m (cons 3 1)))
  (apply * (map (lambda (p) (solve m p))
                (list (cons 1 1) (cons 3 1) (cons 5 1) (cons 7 1) (cons 1 2)))))
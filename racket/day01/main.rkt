#lang racket

(define (solve l)
  (apply * (findf (λ (v) (equal? 2020 (apply + v)))
                  (remove (λ (v) (apply = v))
                          (apply cartesian-product l)))))

(module* main #f
  (define input (map string->number (port->lines)))
  (solve (list input input))
  (solve (list input input input)))
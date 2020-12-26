#lang racket

(struct rule (min max what) #:transparent)

(define (string->histogram s)
  (let ([h (make-hash)])
    (for ([c s]) (hash-update! h c add1 0))
    h))

(define (check r s)
  (define n (hash-ref (string->histogram s) (rule-what r) 0))
  (and (>= n (rule-min r))
       (<= n (rule-max r))))

(define (check-2 r s)
  (xor (equal? (rule-what r) (string-ref s (sub1 (rule-min r))))
       (equal? (rule-what r) (string-ref s (sub1 (rule-max r))))))

(define (parse line)
  (match-define (list all min max what pass) (regexp-match "(.*)-(.*) (.): (.*)" line))
  (cons (rule (string->number min) (string->number max) (string-ref what 0)) pass))

(module* main #f
  (define lines (map parse (port->lines)))
  (length (filter (lambda (l) (check (car l) (cdr l))) lines))
  (length (filter (lambda (l) (check-2 (car l) (cdr l))) lines)))
package fm2gp_test

import (
	"fm2gp"
	"testing"
)

func TestEvaluatePolynomial(t *testing.T) {
	polynomial := []float64{5, 2, 3}
	x := 4.0
	expected := 61.0
	result := fm2gp.EvaluatePolynomial(polynomial, x)
	if result != expected {
		t.Errorf("Expected %v but got %v", expected, result)
	}
}

func TestHornerEvaluatePolynomial(t *testing.T) {
	polynomial := []float64{5, 2, 3}
	x := 4.0
	expected := 61.0
	result := fm2gp.HornerEvaluatePolynomial(polynomial, x)
	if result != expected {
		t.Errorf("Expected %v but got %v", expected, result)
	}
}

func BenchmarkEvaluatePolynomial(b *testing.B) {
	polynomial := []float64{1, 2, 3, 4, 5}
	for i := 0; i < b.N; i++ {
		_ = fm2gp.EvaluatePolynomial(polynomial, 1.23)
	}
}

func BenchmarkHornerEvaluatePolynomial(b *testing.B) {
	polynomial := []float64{1, 2, 3, 4, 5}
	for i := 0; i < b.N; i++ {
		_ = fm2gp.HornerEvaluatePolynomial(polynomial, 1.23)
	}
}

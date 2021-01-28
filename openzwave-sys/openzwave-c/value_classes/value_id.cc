#include "value_id.h"

extern "C" {

// Comparison Operators
bool value_id_eq(ValueID * self, ValueID * other) {
  return *self == *other;
}

bool value_id_less_than(ValueID * self, ValueID * other) {
  return *self < *other;
}

}  // extern "C"

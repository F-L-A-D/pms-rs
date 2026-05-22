#!/usr/bin/env bash

extract_id() {
  sed -n 's/.*"id":"\([^"]*\)".*/\1/p'
}

extract_folio_id() {
  sed -n 's/.*"folio_id":"\([^"]*\)".*/\1/p'
}

extract_receivable_id() {
  sed -n 's/.*"receivable_id":"\([^"]*\)".*/\1/p'
}

extract_allocation_id() {
  sed -n 's/.*"id":"\([^"]*\)".*/\1/p'
}
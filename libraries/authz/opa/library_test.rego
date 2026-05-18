package library.authz_test

import data.library.authz
import rego.v1

# ---------------------------------------------------------------------------
# Fixtures — mirrors the tuples in tests.fga.yaml
# ---------------------------------------------------------------------------

tuples := {
    "system": {"library": {
        "admin": {"user": {"alice": true}},
    }},
    "group": {
        "editors": {
            "admin":  {"user": {"alice": true}},
            "member": {"user": {"carol": true}},
        },
        "readers": {
            "member": {"user": {"dave": true}},
        },
    },
    "book": {
        "1": {
            "owner":  {"user": {"alice": true}},
            "writer": {"user": {"bob": true}},
            "reader": {"group#member": {"readers": true}},
        },
        "2": {
            "writer": {"group#member": {"editors": true}},
        },
    },
    "review": {
        "1": {
            "owner":  {"user": {"bob": true}},
            "writer": {"group#member": {"editors": true}},
        },
        "2": {
            "owner": {"user": {"carol": true}},
        },
    },
}

# Helper: run a check with the fixture data injected.
check(user, relation, object) if authz.allow with input as {
    "user": user, "relation": relation, "object": object,
} with data.library.tuples as tuples

# ---------------------------------------------------------------------------
# system admin
# ---------------------------------------------------------------------------

test_system_admin_alice if check("user:alice", "admin", "system:library")

test_system_admin_bob_denied if not check("user:bob", "admin", "system:library")

# ---------------------------------------------------------------------------
# group admin implies membership
# ---------------------------------------------------------------------------

test_group_admin_is_admin   if check("user:alice", "admin",  "group:editors")
test_group_admin_is_member  if check("user:alice", "member", "group:editors")
test_group_direct_member    if check("user:carol", "member", "group:editors")
test_group_member_not_admin if not check("user:carol", "admin", "group:editors")
test_group_unrelated_denied if not check("user:bob", "member", "group:editors")

# ---------------------------------------------------------------------------
# book owner gets all permissions
# ---------------------------------------------------------------------------

test_book_owner_has_owner  if check("user:alice", "owner",  "book:1")
test_book_owner_has_writer if check("user:alice", "writer", "book:1")
test_book_owner_has_reader if check("user:alice", "reader", "book:1")

# ---------------------------------------------------------------------------
# direct book writer gets writer and reader, not owner
# ---------------------------------------------------------------------------

test_book_writer_has_writer    if check("user:bob", "writer", "book:1")
test_book_writer_has_reader    if check("user:bob", "reader", "book:1")
test_book_writer_not_owner     if not check("user:bob", "owner", "book:1")

# ---------------------------------------------------------------------------
# group member inherits book access via group relation
# ---------------------------------------------------------------------------

# alice: editors admin → member → writer on book:2
test_group_admin_book_writer if check("user:alice", "writer", "book:2")
test_group_admin_book_reader if check("user:alice", "reader", "book:2")

# carol: editors member → writer on book:2
test_group_member_book_writer if check("user:carol", "writer", "book:2")
test_group_member_book_reader if check("user:carol", "reader", "book:2")

# dave: readers member → reader on book:1, not writer
test_group_readers_book_reader     if check("user:dave", "reader", "book:1")
test_group_readers_not_book_writer if not check("user:dave", "writer", "book:1")
test_group_readers_not_book_owner  if not check("user:dave", "owner",  "book:1")

# dave has no relation to book:2
test_group_readers_no_access_book2 if not check("user:dave", "reader", "book:2")

# ---------------------------------------------------------------------------
# unrelated user has no book access
# ---------------------------------------------------------------------------

test_unrelated_no_owner  if not check("user:eve", "owner",  "book:1")
test_unrelated_no_writer if not check("user:eve", "writer", "book:1")
test_unrelated_no_reader if not check("user:eve", "reader", "book:1")

# ---------------------------------------------------------------------------
# review owner gets all permissions
# ---------------------------------------------------------------------------

test_review_owner_has_owner  if check("user:bob", "owner",  "review:1")
test_review_owner_has_writer if check("user:bob", "writer", "review:1")
test_review_owner_has_reader if check("user:bob", "reader", "review:1")

# ---------------------------------------------------------------------------
# group member inherits review writer access
# ---------------------------------------------------------------------------

# alice: editors admin → member → writer on review:1
test_group_admin_review_writer if check("user:alice", "writer", "review:1")
test_group_admin_review_reader if check("user:alice", "reader", "review:1")
test_group_admin_review_not_owner if not check("user:alice", "owner", "review:1")

# carol: editors member → writer on review:1
test_group_member_review_writer if check("user:carol", "writer", "review:1")
test_group_member_review_reader if check("user:carol", "reader", "review:1")

# dave: readers member, no relation to review:1
test_readers_no_review_writer if not check("user:dave", "writer", "review:1")
test_readers_no_review_reader if not check("user:dave", "reader", "review:1")

# ---------------------------------------------------------------------------
# carol owns review:2
# ---------------------------------------------------------------------------

test_carol_review2_owner  if check("user:carol", "owner",  "review:2")
test_carol_review2_writer if check("user:carol", "writer", "review:2")
test_carol_review2_reader if check("user:carol", "reader", "review:2")

test_bob_review2_not_owner  if not check("user:bob", "owner",  "review:2")
test_bob_review2_not_writer if not check("user:bob", "writer", "review:2")
test_bob_review2_not_reader if not check("user:bob", "reader", "review:2")

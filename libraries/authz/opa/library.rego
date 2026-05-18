package library.authz

import rego.v1

# Data layout written by OpaClient::write_tuple:
#   data.library.tuples[<object_type>][<object_id>][<relation>][<user_type>][<user_id>] = true
#
# input:
#   user     string  e.g. "user:alice-uuid"
#   relation string  e.g. "reader"
#   object   string  e.g. "book:some-uuid"

default allow := false

allow if has_tuple(input.user, input.relation, input.object)

# ---------------------------------------------------------------------------
# owner ⊆ writer ⊆ reader  (book and review types share the same hierarchy)
# ---------------------------------------------------------------------------

allow if {
    input.relation == "writer"
    has_tuple(input.user, "owner", input.object)
}

allow if {
    input.relation == "reader"
    has_tuple(input.user, "owner", input.object)
}

allow if {
    input.relation == "reader"
    has_tuple(input.user, "writer", input.object)
}

# Group admin implies member
allow if {
    input.relation == "member"
    [_, group_id] := split_colon(input.object)
    [usr_type, usr_id] := split_colon(input.user)
    data.library.tuples["group"][group_id]["admin"][usr_type][usr_id] == true
}

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

# Direct user tuple match
has_tuple(user, relation, object) if {
    [obj_type, obj_id] := split_colon(object)
    [usr_type, usr_id] := split_colon(user)
    data.library.tuples[obj_type][obj_id][relation][usr_type][usr_id] == true
}

# Group-expanded tuple match
has_tuple(user, relation, object) if {
    [obj_type, obj_id] := split_colon(object)
    some group_id
    data.library.tuples[obj_type][obj_id][relation]["group#member"][group_id] == true
    is_group_member(user, group_id)
}

is_group_member(user, group_id) if {
    [usr_type, usr_id] := split_colon(user)
    data.library.tuples["group"][group_id]["member"][usr_type][usr_id] == true
}

is_group_member(user, group_id) if {
    # admin ⊆ member
    [usr_type, usr_id] := split_colon(user)
    data.library.tuples["group"][group_id]["admin"][usr_type][usr_id] == true
}

split_colon(s) := [type, id] if {
    idx := indexof(s, ":")
    idx >= 0
    type := substring(s, 0, idx)
    id := substring(s, idx + 1, -1)
}

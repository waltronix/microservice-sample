Feature: Book access control

  Scenario: A book owner can read and delete their book
    Given user "alice" creates a book
    Then user "alice" can read the book
    And user "alice" can delete the book

  Scenario: An unrelated user cannot read or delete a book
    Given user "alice" creates a book
    Then user "bob" cannot read the book
    And user "bob" cannot delete the book

  Scenario: A user granted reader access can read but not delete
    Given user "alice" creates a book
    And user "alice" grants user "bob" "reader" access to the book
    Then user "bob" can read the book
    And user "bob" cannot delete the book

  Scenario: A user granted writer access can read and delete
    Given user "alice" creates a book
    And user "alice" grants user "bob" "writer" access to the book
    Then user "bob" can read the book
    And user "bob" can delete the book

  Scenario: A group member inherits reader access via group grant
    Given user "alice" is a system admin
    And user "alice" creates a group
    And user "alice" adds user "bob" to the group
    And user "carol" creates a book
    And user "carol" grants the group "reader" access to the book
    Then user "bob" can read the book
    And user "bob" cannot delete the book

  Scenario: A group member inherits writer access via group grant
    Given user "alice" is a system admin
    And user "alice" creates a group
    And user "alice" adds user "bob" to the group
    And user "carol" creates a book
    And user "carol" grants the group "writer" access to the book
    Then user "bob" can read the book
    And user "bob" can delete the book

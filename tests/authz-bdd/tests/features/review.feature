Feature: Review access control

  Scenario: A book reader can create a review
    Given user "alice" creates a book
    And user "alice" grants user "bob" "reader" access to the book
    When user "bob" creates a review for the book
    Then the response status is 201

  Scenario: A user without book access cannot create a review
    Given user "alice" creates a book
    When user "bob" creates a review for the book
    Then the response status is 403

  Scenario: A review owner can delete their review
    Given user "alice" creates a book
    And user "alice" grants user "bob" "reader" access to the book
    And user "bob" creates a review for the book
    When user "bob" deletes the review
    Then the response status is 204

  Scenario: An unrelated user cannot delete a review
    Given user "alice" creates a book
    And user "alice" grants user "bob" "reader" access to the book
    And user "bob" creates a review for the book
    When user "carol" deletes the review
    Then the response status is 403

  Scenario: A book reader can list reviews
    Given user "alice" creates a book
    And user "alice" grants user "bob" "reader" access to the book
    And user "bob" creates a review for the book
    Then user "bob" can list reviews for the book

  Scenario: A user without book access cannot list reviews
    Given user "alice" creates a book
    And user "alice" grants user "bob" "reader" access to the book
    And user "bob" creates a review for the book
    Then user "carol" cannot list reviews for the book

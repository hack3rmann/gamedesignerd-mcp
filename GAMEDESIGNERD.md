# Game Designer MCP - How to Use

This document provides instructions on how to use the Game Designer MCP tool for AI-assisted game development.

## Overview

The Game Designer MCP is a tool that helps break down game development into small, manageable tasks. It maintains the overall game design and provides focused features one at a time, preventing context pollution and helping you stay focused on one implementation task at a time.

## Core Tools

### `designOverview(sessionName)`
Retrieves the complete game design document for your project. This gives you the overall vision, mechanics, story, and technical requirements for the game.

### `nextFeature(sessionName)`
Gets the next small, focused feature to implement. Each feature is designed to be a single, manageable task that can be completed quickly (e.g., a single function, small component, basic UI element).

### `featureReview(sessionName, changesMade)`
Submits your implementation report for review. Provide a detailed description of what you implemented, including key technical decisions and any challenges encountered.

### `reviewReply(sessionName, content)`
Responds to questions from the designer about your implementation. Use this to clarify details or address feedback.

### `featureAsk(sessionName, question)`
Asks ad-hoc questions about the current feature or overall design. Use this when you need clarification on implementation details or design decisions.

## Workflow

The typical workflow when using the Game Designer MCP:

1. **Get Design Overview**: Start by understanding the complete game design
2. **Get Next Feature**: Retrieve the first small feature to implement
3. **Implement**: Focus on implementing just that one feature
4. **Review**: Submit your implementation for review
5. **Respond**: Answer any questions from the review
6. **Repeat**: Get the next feature and continue the cycle

## Working with Features

### Feature Characteristics
- Features are small and focused
- Each feature should be completable in a short time
- Features build incrementally toward the complete game
- Features include implementation details and technical requirements

### Implementation Reports
When submitting a `featureReview`, provide:
- What you implemented
- Key technical decisions made
- Any challenges encountered
- How it fits into the overall design
- Specific details about the implementation approach

### Asking Questions
Use `featureAsk` when you need:
- Clarification on implementation details
- Guidance on technical approaches
- Information about how features connect
- Help with design decisions

## Best Practices

1. **Focus on One Feature**: Complete one feature entirely before moving to the next
2. **Be Detailed in Reviews**: Provide comprehensive information about your implementation
3. **Ask Specific Questions**: When using `featureAsk`, be precise about what you need to know
4. **Keep Implementations Small**: The tool works best when features are truly incremental
5. **Respond Promptly**: Address review questions quickly to maintain momentum

## Example Workflow

1. **Start**: Use `designOverview` to understand the complete game design
2. **First Feature**: Use `nextFeature` to get your first implementation task
3. **Implement**: Focus solely on implementing that feature
4. **Review**: Use `featureReview` to submit your implementation report
5. **Answer Questions**: Use `reviewReply` if the designer has questions
6. **Next Feature**: Use `nextFeature` to get the next task
7. **Continue**: Repeat steps 3-6 until the game is complete

## Getting Help

Each tool provides detailed information about its parameters and usage. The designer will guide you through the process and provide specific feedback on your implementations.

This workflow allows you to focus on one small implementation task at a time while the Game Designer MCP maintains the overall design coherence and project state.
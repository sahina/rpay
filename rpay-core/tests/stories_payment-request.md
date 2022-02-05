# Payment Request Stories

## Feature: Payment Request

### Create new payment request

```gherkin
Given a client
When submits new payment request
Then payment request is created
And email notification is sent to the recipient
And sms notification is sent to the recipient
And payment request is set to `created` state
```

todo: invalid stories ?

### Cancel payment request

```gherkin
Given a client
When cancels a payment request
Then payment request is cancelled
And email notification is sent to the recipient
And sms notification is sent to the recipient
```
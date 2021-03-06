use clinvoice_finance::{Currency, ExchangeRates, Exchangeable};

use super::MatchTimesheet;

impl Exchangeable for MatchTimesheet
{
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			expenses: self.expenses.exchange(currency, rates),
			job: self.job.exchange(currency, rates),
			..self
		}
	}

	fn exchange_ref(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			employee: self.employee.clone(),
			expenses: self.expenses.exchange_ref(currency, rates),
			id: self.id.clone(),
			job: self.job.exchange_ref(currency, rates),
			time_begin: self.time_begin.clone(),
			time_end: self.time_end.clone(),
			work_notes: self.work_notes.clone(),
		}
	}
}

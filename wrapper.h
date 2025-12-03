/* wrapper.h - Headers for gnucash-sys FFI bindings */

/* Core types */
#include "guid.h"
#include "gnc-numeric.h"
#include "gnc-date.h"

/* Engine and session */
#include "gnc-engine.h"
#include "qofsession.h"

/* Entity types */
#include "qofbook.h"
#include "Account.h"
#include "Transaction.h"
#include "Split.h"

/* Price database */
#include "gnc-pricedb.h"

/* Query framework */
#include "qofquery.h"
#include "qofquerycore.h"

/* Business module */
#include "gncAddress.h"
#include "gncCustomer.h"
#include "gncVendor.h"
#include "gncEmployee.h"
#include "gncJob.h"
#include "gncInvoice.h"
#include "gncEntry.h"
#include "gncBillTerm.h"
#include "gncTaxTable.h"
#include "gncOwner.h"

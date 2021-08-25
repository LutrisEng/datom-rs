#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Network/disk errors
 */
typedef enum DatomConnectionError {
  /**
   * No error
   */
  None,
  /**
   * There was invalid data in the data store
   */
  InvalidData,
  /**
   * There was an IO error
   */
  IOError,
  /**
   * There was an other error, possibly in the underlying data
   * store
   */
  Miscellaneous,
} DatomConnectionError;

/**
 * An un-resolved entity [ID], which can be used to resolve entities by [ident](crate::builtin_idents::ident) or [unique](crate::builtin_idents::unique) attribute
 */
typedef struct EID EID;

/**
 * A persistent [Connection] to a sled-backed database
 */
typedef struct SledConnection SledConnection;

/**
 * A view of a sled-backed database
 */
typedef struct SledDatabase SledDatabase;

/**
 * A set of facts which can be transacted into a database connection
 */
typedef struct Transaction Transaction;

/**
 * An attribute value.
 */
typedef struct Value Value;

/**
 * The result of running a [Transaction](crate::Transaction) on a
 * [Connection]; for C bindings
 */
typedef struct DatomTransactionResult_SledConnection__SledDatabase {
  /**
   *     The [Connection] the [Transaction](crate::Transaction) was run     on
   */
  const struct SledConnection *connection;
  /**
   * The [database](crate::database::Database) before the transaction
   */
  struct SledDatabase *before;
  /**
   * The [database](crate::database::Database) after the transaction
   */
  struct SledDatabase *after;
} DatomTransactionResult_SledConnection__SledDatabase;

/**
 * Create a connection to a sled-backed database at the given path
 *
 * # Safety
 *
 * path must be a NULL-terminated string. You must call
 * [datom_sled_disconnect] when you are done with the
 * SledConnection. Returns NULL on an error - check
 * [datom_last_connection_error] to get the error code.
 */
struct SledConnection *datom_sled_connect(const char *path);

/**
 * Destroy a connection to a sled-backed database
 *
 * # Safety
 *
 * conn must be a valid, non-null [SledConnection] created by
 * [datom_sled_connect].
 */
void datom_sled_disconnect(struct SledConnection *conn);

/**
 * Run a transaction on a sled-backed database. Consumes
 * transaction.
 *
 * # Safety
 *
 * conn must be a valid, non-null [SledConnection] created by
 * [datom_sled_connect]. transaction must be a valid, non-null
 * [Transaction] created by [datom_transaction_create]. You must
 * destroy the return value (if non-NULL) after you are done.
 */
struct DatomTransactionResult_SledConnection__SledDatabase *datom_sled_transact(const struct SledConnection *conn,
                                                                                struct Transaction *transaction);

/**
 * Get a [database](SledDatabase) for the current point in time
 */
struct SledDatabase *datom_sled_db(const struct SledConnection *conn);

/**
 * Get a [database](SledDatabase) for a specific point in time
 */
struct SledDatabase *datom_sled_as_of(const struct SledConnection *conn, uint64_t t);

/**
 * Get the last [DatomConnectionError] thrown
 */
enum DatomConnectionError datom_last_connection_error(void);

/**
 * Destroy a connection to a sled-backed database view
 *
 * # Safety
 *
 * db must be a valid, non-null [SledDatabase] created by
 * [datom_sled_db](crate::c::datom_sled_db).
 */
void datom_sled_db_destroy(struct SledDatabase *db);

/**
 * Create an EID object from a 16-byte ID.
 */
struct EID *datom_eid_id(const uint8_t (*id)[16]);

/**
 * Create an EID object from a string ident.
 *
 * # Safety
 *
 * ident must be a NULL-terminated string.
 */
struct EID *datom_eid_ident(const char *ident);

/**
 * Create an EID object from an attribute and value. Consumes attr
 * and value.
 */
struct EID *datom_eid_unique(struct EID *attr, struct Value *value);

/**
 * Destroy an EID object which wasn't consumed
 *
 * # Safety
 *
 * eid must be a valid, non-null [EID] created by [datom_eid_id],
 * [datom_eid_ident], or [datom_eid_unique].
 */
void datom_eid_destroy(struct EID *eid);

/**
 * Create a transaction object
 */
struct Transaction *datom_transaction_create(void);

/**
 * Add an attribute value to an entity in a transaction
 *
 * Consumes entity, attribute, and value.
 *
 * # Safety
 *
 * transaction must be a valid, non-null [Transaction] created by
 * [datom_transaction_create].
 */
void datom_transaction_add(struct Transaction *transaction,
                           struct EID *entity,
                           struct EID *attribute,
                           struct Value *value);

/**
 * Retract a specific attribute value from an entity in a transaction
 *
 * Consumes entity, attribute, and value.
 *
 * # Safety
 *
 * transaction must be a valid, non-null [Transaction] created by
 * [datom_transaction_create].
 */
void datom_transaction_retract_value(struct Transaction *transaction,
                                     struct EID *entity,
                                     struct EID *attribute,
                                     struct Value *value);

/**
 * Retract an attribute from an entity, ignoring its value, in a
 * transaction
 *
 * Consumes entity and attribute.
 *
 * # Safety
 *
 * transaction must be a valid, non-null [Transaction] created by
 * [datom_transaction_create].
 */
void datom_transaction_retract(struct Transaction *transaction,
                               struct EID *entity,
                               struct EID *attribute);

/**
 * Destroy a transaction object which wasn't consumed
 *
 * # Safety
 *
 * transaction must be a valid, non-null [Transaction] created by
 * [datom_transaction_create].
 */
void datom_transaction_destroy(struct Transaction *transaction);

/**
 * Destroy a transaction result
 *
 * # Safety
 *
 * res must be a valid, non-null [DatomTransactionResult] created by
 * [datom_sled_transact](crate::c::datom_sled_transact).
 */
void datom_sled_transaction_result_destroy(struct DatomTransactionResult_SledConnection__SledDatabase *res);

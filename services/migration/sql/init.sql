CREATE TABLE raw_txn
(
    signature varchar(64) PRIMARY KEY,
    slot      bigint not null,
    processed bool   not null
);
-- @@@@@@

CREATE INDEX raw_slot on raw_txn (slot);
-- @@@@@@

CREATE TABLE cl_items
(
    id       bigserial PRIMARY KEY,
    tree     BYTEA  NOT NULL,
    node_idx BIGINT NOT NULL,
    leaf_idx BIGINT,
    seq      BIGINT NOT NULL,
    level    BIGINT NOT NULL,
    hash     BYTEA  NOT NULL
);
-- @@@@@@
-- Index All the things space is cheap
CREATE INDEX cl_items_tree_idx on cl_items (tree);
-- @@@@@@
CREATE INDEX cl_items_hash_idx on cl_items (hash);
-- @@@@@@
CREATE INDEX cl_items_level on cl_items (level);
-- @@@@@@
CREATE INDEX cl_items_node_idx on cl_items (node_idx);
-- @@@@@@
CREATE INDEX cl_items_leaf_idx on cl_items (leaf_idx);
-- @@@@@@
CREATE UNIQUE INDEX cl_items__tree_node on cl_items (tree, node_idx);
-- @@@@@@

CREATE TABLE backfill_items
(
    id         bigserial PRIMARY KEY,
    tree       BYTEA    not null,
    seq        BIGINT   not null,
    slot       BIGINT   not null,
    force_chk  bool     not null,
    backfilled bool     not null,
    failed     bool     not null default false,
    locked     bool     not null default false
);
-- @@@@@@

CREATE INDEX backfill_items_tree_idx on backfill_items (tree);
-- @@@@@@
CREATE INDEX backfill_items_seq_idx on backfill_items (seq);
-- @@@@@@
CREATE INDEX backfill_items_slot_idx on backfill_items (slot);
-- @@@@@@
CREATE INDEX backfill_items_force_chk_idx on backfill_items (force_chk);
-- @@@@@@
CREATE INDEX backfill_items_backfilled_idx on backfill_items (backfilled);
-- @@@@@@
CREATE INDEX backfill_items_tree_seq_idx on backfill_items (tree, seq);
-- @@@@@@
CREATE INDEX backfill_items_tree_slot_idx on backfill_items (tree, slot);
-- @@@@@@
CREATE INDEX backfill_items_tree_force_chk_idx on backfill_items (tree, force_chk);
-- @@@@@@
CREATE INDEX backfill_items_tree_backfilled_idx on backfill_items (tree, backfilled);
-- @@@@@@
CREATE INDEX backfill_items_failed_idx on backfill_items (failed);
-- @@@@@@
CREATE INDEX backfill_items_tree_failed_idx on backfill_items (tree, failed);
-- @@@@@@
CREATE INDEX backfill_items_locked_idx on backfill_items (locked);
-- @@@@@@
CREATE INDEX backfill_items_tree_locked_idx on backfill_items (tree, locked);
-- @@@@@@

CREATE
    or REPLACE FUNCTION notify_new_backfill_item()
    RETURNS trigger
    LANGUAGE 'plpgsql'
as
$BODY$
declare
begin
    if
        (tg_op = 'INSERT') then
        perform pg_notify('backfill_item_added', 'hello');

    end if;

    return null;
end
$BODY$;
-- @@@@@@

CREATE TRIGGER after_insert_item
    AFTER INSERT
    ON backfill_items
    FOR EACH ROW
EXECUTE PROCEDURE notify_new_backfill_item();

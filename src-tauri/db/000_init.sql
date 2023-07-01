-- columnテーブル作成
CREATE TABLE IF NOT EXISTS columns (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL
);

-- cardsテーブル作成
CREATE TABLE IF NOT EXISTS cards (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT
);

-- columns_cardsテーブルを作成する
CREATE TABLE IF NOT EXISTS columns_cards (
    column_id INTEGER NOT NULL,
    card_id INTEGER NOT NULL,
    card_position INTEGER NOT NULL,
    PRIMARY KEY (column_id, card_id),
    FOREIGN KEY (column_id) REFERENCES columns (id) ON DELETE CASCADE,
    FOREIGN KEY (card_id) REFERENCES cards (id) ON DELETE CASCADE
);

-- columnsテーブルにサンプルデータを挿入する
INSERT INTO columns (id, title) VALUES (0, '課題一覧');
INSERT INTO columns (id, title) VALUES (1, '制作中');
INSERT INTO columns (id, title) VALUES (2, '保留中');
INSERT INTO columns (id, title) VALUES (3, '提出完了');

-- cardsテーブルにサンプルデータを挿入する
INSERT INTO cards (id, title) VALUES (0, '実験レポート');
INSERT INTO cards (id, title) VALUES (1, '報告書作成');
INSERT INTO cards (id, title, description) VALUES (2, 'エントリーシート作成', '優先：チームラボ');
INSERT INTO cards (id, title) VALUES (3, 'tauriアプリ完成');
INSERT INTO cards (id, title, description) VALUES (4, 'かんばんのデータをSQLiteに保存する', 'SQLxクレートを使用する');
INSERT INTO cards (id, title) VALUES (5, 'アプリのインストーラをビルドする');

-- columns_cardsテーブルにサンプルデータを挿入する
INSERT INTO columns_cards (column_id, card_id, card_position) VALUES (3, 0, 0);
INSERT INTO columns_cards (column_id, card_id, card_position) VALUES (3, 1, 1);
INSERT INTO columns_cards (column_id, card_id, card_position) VALUES (1, 2, 0);
INSERT INTO columns_cards (column_id, card_id, card_position) VALUES (0, 3, 0);
INSERT INTO columns_cards (column_id, card_id, card_position) VALUES (0, 4, 1);
INSERT INTO columns_cards (column_id, card_id, card_position) VALUES (2, 5, 0);
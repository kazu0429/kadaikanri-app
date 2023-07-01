// @ts-ignore
import Board from '@asseinfo/react-kanban';
import '@asseinfo/react-kanban/dist/styles.css';
import "./App.css";

import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";

type TBoard = {
  columns:[TColumn];
}

type TColumn = {
  id:number;
  title:string;
  card:[TCard];
}

type TCard = {
  id:number;
  title:string;
  description:string | undefined;
}

type TMovedForm = {
  fromColumnId:number;
  fromPosotion:number;
}

type TMovedTo = {
  toColumnId:number;
  toPosition:number;
}

class CardPos {
  columnId:number;
  position:number;

  constructor(columnId:number, posotion:number){
    this.columnId = columnId;
    this.position = posotion;
  }
}

async function handleAddCard(board:TBoard, column:TColumn, card:TCard){
  const pos = new CardPos(column.id, 0);
  await invoke<void>("handle_add_card", {"card":card, "pos":pos})
}

async function handleMoveCaard(board:TBoard, card:TCard, from:TMovedForm, to:TMovedTo){
  const fromPos = new CardPos(from.fromColumnId, from.fromPosotion);
  const toPos = new CardPos(to.toColumnId, to.toPosition);
  await invoke<void>("handle_move_card", {"card":card, "from":fromPos, "to":toPos});
}

async function handleRemoveCard(board:TBoard, column:TColumn, card:TCard){
  await invoke<void>("handle_remove_card", {"card":card, "columnId":column.id});
}


function App() {

  const [board, setBoard] = useState<TBoard | null>(null);

  useEffect(() => {
    (async() => {
      try{
        const board = await invoke<TBoard>("get_board", {});
        console.log(board);
        setBoard(board);
      }catch(err){
        console.error(err);
        return null;
      }
    })()
  },[])


  return (
    <>
      {board != null &&
        <Board
          // ボードの初期データ
          initialBoard={board}
          // カードの追加を許可（トップに「＋」ボタンを表示）
          allowAddCard={{ on: "top" }}
          // カードの削除を許可
          allowRemoveCard
          // カラム（カードのグループ）のドラッグをオフにする
          disableColumnDrag
          // 新しいカードの作成時、idに現在時刻の数値表現をセットする
          onNewCardConfirm={(draftCard: any) => ({
            id: new Date().getTime(),
            ...draftCard
          })}
          // 新しいカードが作成されたら、カード等の内容をコンソールに表示する
          onCardNew={handleAddCard}
          // カードがドラッグされたら、カード等の内容をコンソールに表示する
          onCardDragEnd={handleMoveCaard}
          // カードが削除されたら、カード等の内容をコンソールに表示する
          onCardRemove={handleRemoveCard}
        />
      }
    </>
  );
}

export default App;

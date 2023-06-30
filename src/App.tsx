import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
// @ts-ignore
import Board from '@asseinfo/react-kanban';
import '@asseinfo/react-kanban/dist/styles.css';
import "./App.css";

const board = {
  columns:[
    {
      id:0,
      title:"課題一覧",
      cards:[
        {
          id:0,
          title:"実験レポート",
          description:"Apacheの調査"
        },{
          id:1,
          title:"応用課題を進める",
          description:"ポスタライズについて"
        }
      ]
    },
    {
      id:0,
      title:"制作中",
      cards:[]
    }
  ]
}

function App() {
  return (
    <>
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
        onCardNew={console.log}
        // カードがドラッグされたら、カード等の内容をコンソールに表示する
        onCardDragEnd={console.log}
        // カードが削除されたら、カード等の内容をコンソールに表示する
        onCardRemove={console.log}
      />
    </>
  );
}

export default App;

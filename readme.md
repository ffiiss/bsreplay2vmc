# bsreplay2vmc

ScoreSaber のリプレイファイルを読み込み、 [VirtualMotionCapture](https://vmc.info/) に送るプログラムです。

もともとは[この記事](https://note.com/ffiiss/n/n7ba3d7ed1258)を書くために、リプレイファイルを解析して分布を調べようと作り始めた覚えがありますが、関係ないものができました。個人的な解析プログラムの副産物なので UI などはありません。

## 使い方

1. [ここ](https://github.com/ffiiss/bsreplay2vmc/releases)から `bs2replay2vmc.exe` をダウンロードしてください。たぶんウィルスの警告とか出るだろうなあ。

2. VirtualMotionCapture を起動し、 VMC プロトコルの受信を有効にします。

3,. `bsreplay2vmc.exe` にリプレイファイルをドラッグ & ドロップするか、コマンドラインで引数に与えて起動してください。複数ファイル指定も可能です。

自分のリプレイファイルはたとえば `C:/Program Files (x86)/Steam/steamapps/common/Beat Saber/UserData/ScoreSaber/Replays/` にあります。 ScoreSaber のデータは `https://scoresaber.com/game/replays/[Song ID]-[Player ID].dat` に存在し…たと思うのですが、いま確認したらこの URL はなくなっている気がします。

最初にキャリブレーションを行いますが、コントローラー軸は今のところ未補正なので、持ち方/コントローラの種類によっては大変なことになります。

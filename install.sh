cargo build --release

mkdir -p ~/.local/bin/
cp -i target/release/pyvenvselect ~/.local/bin
mkdir -p ~/.config/pyvenvselect
cp -i .config/pyvenvselect/* ~/.config/pyvenvselect

echo Creating '~/.config/pyvenvselect/pyvenvactivate.bash'
echo "### pyvenvselect" > ~/.config/pyvenvselect/pyvenvactivate.bash
echo "export PYVENV_CURRENT=~/.config/pyvenvselect/current" >> ~/.config/pyvenvselect/pyvenvactivate.bash


echo Creating '~/.config/pyvenvselect/pyvenvactivate.zsh'
echo "### pyvenvselect" > ~/.config/pyvenvselect/pyvenvactivate.zsh
echo "export PYVENV_CURRENT=~/.config/pyvenvselect/current" >> ~/.config/pyvenvselect/pyvenvactivate.zsh

echo Creating '~/.config/pyvenvselect/pyvenvactivate.fish'
echo "### pyvenvselect" > ~/.config/pyvenvselect/pyvenvactivate.fish
echo "set -U PYVENV_CURRENT ~/.config/pyvenvselect/current" >> ~/.config/pyvenvselect/pyvenvactivate.fish

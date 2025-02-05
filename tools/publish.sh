# if crate A depends on crate B, B must come before A in this list
crates=(
)

if [ -n "$(git status --porcelain)" ]; then
    echo "You have local changes!"
    exit 1
fi

pushd crates

for crate in "${crates[@]}"; do
    echo "Publishing ${crate}"
    cp ../LICENSE-MIT "$crate"
    cp ../LICENSE-APACHE "$crate"
    pushd "$crate"
    git add LICENSE-MIT LICENSE-APACHE
    cargo publish --no-verify --allow-dirty
    popd
    sleep 20
done

popd

echo "Publishing root crate"
cargo publish --allow-dirty

echo "Cleaning local state"
git reset HEAD --hard

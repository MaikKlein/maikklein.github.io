gutenberg build && \
git branch -D gh-pages && \
git checkout -b gh-pages && \
cp -r public/* . && \
git add . && \
git commit -m "publish" && \
git checkout master

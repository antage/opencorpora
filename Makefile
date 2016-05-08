.PHONY: doc push

doc:
	cargo doc
	echo "<meta http-equiv=refresh content=0;url=https://antage.github.io/opencorpora/opencorpora/index.html>" > target/doc/index.html
	ghp target/doc

push: doc
	git push origin master:master
	git push --tags
	git push origin gh-pages:gh-pages

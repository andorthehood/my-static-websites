module Jekyll
	class RenderTimeTag < Liquid::Tag
		def render(context)
			"#{Time.now}"
		end
	end
end

Liquid::Template.register_tag('render_time', Jekyll::RenderTimeTag)
